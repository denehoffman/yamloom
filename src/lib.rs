#![allow(clippy::wrong_self_convention)]
#![allow(clippy::too_many_arguments)]

use std::{
    fmt::Display,
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::Path,
};

use hashlink::LinkedHashMap;
use pyo3::{
    exceptions::{PyRuntimeError, PyValueError},
    prelude::*,
    types::{PyDict, PyDictMethods},
};
use yaml_rust2::{
    yaml::{Array, Hash},
    Yaml, YamlEmitter,
};

pub trait Yamlable {
    fn as_yaml(&self) -> Yaml;
    fn as_yaml_string(&self) -> PyResult<String> {
        let yaml = self.as_yaml();
        let mut out_str = String::new();
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.multiline_strings(true);
        emitter
            .dump(&yaml)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;
        Ok(out_str)
    }
    fn write_to_file(&self, path: impl AsRef<Path>, overwrite: bool) -> PyResult<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty() {
                create_dir_all(parent)?;
            }
        let mut opts = OpenOptions::new();
        opts.write(true).create(true);
        if overwrite {
            opts.truncate(true);
        } else {
            opts.create_new(true);
        }
        let mut file = match opts.open(path) {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => return Ok(()),
            Err(e) => return Err(PyErr::from(e)),
        };

        file.write_all(self.as_yaml_string()?.as_bytes())?;
        file.flush()?;
        Ok(())
    }
}
impl Yamlable for Yaml {
    fn as_yaml(&self) -> Yaml {
        self.clone()
    }
}
impl Yamlable for &Yaml {
    fn as_yaml(&self) -> Yaml {
        self.to_owned().clone()
    }
}

fn push_escaped_control(out: &mut String, ch: char) -> bool {
    match ch {
        '\n' => out.push_str("\\n"),
        '\r' => out.push_str("\\r"),
        '\t' => out.push_str("\\t"),
        c if c.is_control() => {
            use std::fmt::Write as _;
            let _ = write!(out, "\\u{:04X}", c as u32);
        }
        _ => return false,
    }
    true
}

fn escape_control_chars(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        if !push_escaped_control(&mut out, ch) {
            out.push(ch);
        }
    }
    out
}
impl Yamlable for f64 {
    fn as_yaml(&self) -> Yaml {
        Yaml::Real(self.to_string())
    }
}
impl Yamlable for &f64 {
    fn as_yaml(&self) -> Yaml {
        Yaml::Real(self.to_string())
    }
}
impl Yamlable for String {
    fn as_yaml(&self) -> Yaml {
        if self.contains("${{") && self.contains("}}") {
            let escaped = escape_control_chars(self);
            // prevents variables from being quoted when they might
            // evaluate as bools or numbers
            Yaml::Real(escaped)
        } else {
            Yaml::String(self.to_string())
        }
    }
}
impl Yamlable for &str {
    fn as_yaml(&self) -> Yaml {
        self.to_string().as_yaml()
    }
}
impl Yamlable for &String {
    fn as_yaml(&self) -> Yaml {
        self.to_string().as_yaml()
    }
}
impl Yamlable for i64 {
    fn as_yaml(&self) -> Yaml {
        Yaml::Integer(*self)
    }
}
impl Yamlable for &i64 {
    fn as_yaml(&self) -> Yaml {
        Yaml::Integer(**self)
    }
}
impl Yamlable for bool {
    fn as_yaml(&self) -> Yaml {
        Yaml::Boolean(*self)
    }
}
impl Yamlable for &bool {
    fn as_yaml(&self) -> Yaml {
        Yaml::Boolean(**self)
    }
}
impl<T> Yamlable for Vec<T>
where
    T: Yamlable,
{
    fn as_yaml(&self) -> Yaml {
        Yaml::Array(self.iter().map(|x| x.as_yaml()).collect())
    }
}
impl<T> Yamlable for &Vec<T>
where
    T: Yamlable,
{
    fn as_yaml(&self) -> Yaml {
        Yaml::Array(self.iter().map(|x| x.as_yaml()).collect())
    }
}
impl<T> Yamlable for &[T]
where
    T: Yamlable,
{
    fn as_yaml(&self) -> Yaml {
        Yaml::Array(self.iter().map(|x| x.as_yaml()).collect())
    }
}

#[derive(Clone)]
pub struct PyMap<K, V>(LinkedHashMap<K, V>)
where
    K: std::cmp::Eq + std::hash::Hash;
impl<K, V> PyMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    fn iter(&self) -> hashlink::linked_hash_map::Iter<'_, K, V> {
        self.0.iter()
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl<K, V> IntoIterator for PyMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    type Item = <LinkedHashMap<K, V> as IntoIterator>::Item;

    type IntoIter = <LinkedHashMap<K, V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<K, V> Default for PyMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
{
    fn default() -> Self {
        Self(Default::default())
    }
}
impl<'a, 'py, K, V> FromPyObject<'a, 'py> for PyMap<K, V>
where
    K: FromPyObjectOwned<'py> + std::cmp::Eq + std::hash::Hash,
    V: FromPyObjectOwned<'py>,
{
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        let dict = obj.cast::<PyDict>()?;
        let mut ret = LinkedHashMap::with_capacity(dict.len());
        for (k, v) in dict.iter() {
            ret.insert(
                k.extract().map_err(Into::into)?,
                v.extract().map_err(Into::into)?,
            );
        }
        Ok(PyMap(ret))
    }
}
impl<K, V> Yamlable for &PyMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash,
    for<'a> &'a K: Yamlable,
    for<'b> &'b V: Yamlable,
{
    fn as_yaml(&self) -> Yaml {
        let mut hash = Hash::new();
        for (k, v) in self.0.iter() {
            hash.insert_yaml(k, v);
        }
        Yaml::Hash(hash)
    }
}
#[derive(Clone)]
pub enum BoolOrString {
    Bool(bool),
    String(String),
}
impl Yamlable for &BoolOrString {
    fn as_yaml(&self) -> Yaml {
        match self {
            BoolOrString::Bool(b) => Yaml::Boolean(*b),
            BoolOrString::String(s) => Yaml::String(s.clone()),
        }
    }
}
impl<'a, 'py> FromPyObject<'a, 'py> for BoolOrString {
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(b) = obj.extract::<bool>() {
            Ok(Self::Bool(b))
        } else if let Ok(s) = obj.extract::<String>() {
            Ok(Self::String(s))
        } else {
            Err(PyValueError::new_err(
                "Expected bool or string which evaluates to bool",
            ))
        }
    }
}

pub trait MaybeYamlable {
    fn maybe_as_yaml(&self) -> Option<Yaml>;
    fn maybe_as_yaml_string(&self) -> PyResult<String> {
        self.maybe_as_yaml()
            .map(|y| y.as_yaml_string())
            .unwrap_or(Ok("".to_string()))
    }
}

pub trait TryYamlable {
    fn try_as_yaml(&self) -> PyResult<Yaml>;
}

pub trait TryHash {
    fn try_as_hash(&self) -> PyResult<Hash>;
}
pub trait TryArray {
    fn try_as_array(&self) -> PyResult<Vec<Yaml>>;
}
impl<T> TryYamlable for T
where
    T: Yamlable,
{
    fn try_as_yaml(&self) -> PyResult<Yaml> {
        Ok(self.as_yaml())
    }
}
pub trait InsertYaml {
    fn insert_yaml(&mut self, key: impl Yamlable, value: impl Yamlable);
    fn insert_yaml_opt(&mut self, key: impl Yamlable, value: impl IntoYamlOption);
}

pub trait IntoYamlOption {
    fn into_yaml_option(self) -> Option<Yaml>;
}

impl<T> IntoYamlOption for Option<T>
where
    T: Yamlable,
{
    fn into_yaml_option(self) -> Option<Yaml> {
        self.map(|value| value.as_yaml())
    }
}

impl<'a, T> IntoYamlOption for &'a Option<T>
where
    &'a T: Yamlable,
{
    fn into_yaml_option(self) -> Option<Yaml> {
        self.as_ref().map(|value| value.as_yaml())
    }
}

impl InsertYaml for Hash {
    fn insert_yaml(&mut self, key: impl Yamlable, value: impl Yamlable) {
        self.insert(key.as_yaml(), value.as_yaml());
    }
    fn insert_yaml_opt(&mut self, key: impl Yamlable, value: impl IntoYamlOption) {
        if let Some(value) = value.into_yaml_option() {
            self.insert(key.as_yaml(), value);
        }
    }
}

pub trait PushYaml {
    fn push_yaml(&mut self, value: impl Yamlable);
    fn push_yaml_opt(&mut self, value: impl IntoYamlOption);
    fn push_yaml_cond(&mut self, value: impl Yamlable, cond: bool);
}

impl PushYaml for Array {
    fn push_yaml(&mut self, value: impl Yamlable) {
        self.push(value.as_yaml());
    }

    fn push_yaml_opt(&mut self, value: impl IntoYamlOption) {
        if let Some(value) = value.into_yaml_option() {
            self.push(value.as_yaml());
        }
    }
    fn push_yaml_cond(&mut self, value: impl Yamlable, cond: bool) {
        if cond {
            self.push_yaml(value);
        }
    }
}

pub enum Either<A, B> {
    A(A),
    B(B),
}
impl<A, B> Clone for Either<A, B>
where
    A: Clone,
    B: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Self::A(a) => Self::A(a.clone()),
            Self::B(b) => Self::B(b.clone()),
        }
    }
}
impl<A, B> Display for Either<A, B>
where
    A: Display,
    B: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Either::A(a) => write!(f, "{a}"),
            Either::B(b) => write!(f, "{b}"),
        }
    }
}
impl<'a, 'py, A, B> FromPyObject<'a, 'py> for Either<A, B>
where
    A: FromPyObject<'a, 'py>,
    B: FromPyObject<'a, 'py>,
{
    type Error = PyErr;

    fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(a) = obj.extract::<A>() {
            Ok(Self::A(a))
        } else if let Ok(b) = obj.extract::<B>() {
            Ok(Self::B(b))
        } else {
            Err(PyValueError::new_err("Invalid value"))
        }
    }
}
impl<A, B> Yamlable for Either<A, B>
where
    for<'a> &'a A: Yamlable,
    for<'b> &'b B: Yamlable,
{
    fn as_yaml(&self) -> Yaml {
        match self {
            Either::A(a) => a.as_yaml(),
            Either::B(b) => b.as_yaml(),
        }
    }
}
impl<A, B> Yamlable for &Either<A, B>
where
    for<'a> &'a A: Yamlable,
    for<'b> &'b B: Yamlable,
{
    fn as_yaml(&self) -> Yaml {
        match self {
            Either::A(a) => a.as_yaml(),
            Either::B(b) => b.as_yaml(),
        }
    }
}

/// A Pythonic implementation of GitHub Actions syntax
#[pymodule]
#[pyo3(name = "_yamloom")]
mod yamloom {
    use std::{collections::HashMap, fmt::Display, path::PathBuf, str::FromStr};

    use pyo3::{
        exceptions::PyValueError,
        prelude::*,
        types::{PyBool, PyDict, PyFloat, PyInt, PyList, PyString, PyTuple},
    };
    use yaml_rust2::{
        yaml::{Array, Hash},
        Yaml,
    };

    use crate::{
        yamloom::expressions::{
            BooleanExpression, NumberExpression, StringExpression, YamlExpression,
        },
        Either, InsertYaml, MaybeYamlable, PushYaml, PyMap, TryArray, TryHash, TryYamlable,
        Yamlable,
    };

    #[pymodule]
    mod expressions {
        use pyo3::types::{PyFloat, PyInt};

        use crate::push_escaped_control;

        use super::*;

        type StringLike = Either<StringExpression, String>;
        type BoolLike = Either<BooleanExpression, bool>;
        type NumberLike = Either<NumberExpression, f64>;

        fn render_string_like(value: StringLike) -> String {
            match value {
                Either::A(expr) => expr.to_string(),
                Either::B(raw) => escape_string(&raw),
            }
        }

        fn render_bool_like(value: BoolLike) -> String {
            match value {
                Either::A(expr) => expr.to_string(),
                Either::B(raw) => {
                    if raw {
                        "true".to_string()
                    } else {
                        "false".to_string()
                    }
                }
            }
        }

        fn render_number_like(value: NumberLike) -> String {
            match value {
                Either::A(expr) => expr.to_string(),
                Either::B(raw) => raw.to_string(),
            }
        }

        pub trait YamlExpression {
            fn stringify(&self) -> &str;
            fn as_expression_string(&self) -> String {
                format!("${{{{ {} }}}}", self.stringify())
            }
        }
        impl<T> Yamlable for &T
        where
            T: YamlExpression,
        {
            fn as_yaml(&self) -> Yaml {
                Yaml::Real(self.as_expression_string())
            }
        }

        #[pyclass]
        #[derive(Clone)]
        pub struct BooleanExpression(String);
        impl YamlExpression for BooleanExpression {
            fn stringify(&self) -> &str {
                &self.0
            }
        }
        impl Display for BooleanExpression {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.stringify())
            }
        }
        #[pymethods]
        impl BooleanExpression {
            fn as_num(&self) -> NumberExpression {
                NumberExpression(self.to_string())
            }
            fn as_str(&self) -> StringExpression {
                StringExpression(self.to_string())
            }
            fn as_obj(&self) -> ObjectExpression {
                ObjectExpression(self.to_string())
            }
            fn __invert__(&self) -> Self {
                Self(format!("!({})", self))
            }
            fn __and__(&self, other: BoolLike) -> Self {
                let other = render_bool_like(other);
                Self(format!("({} && {})", self, other))
            }
            fn __or__(&self, other: BoolLike) -> Self {
                let other = render_bool_like(other);
                Self(format!("({} || {})", self, other))
            }
            // TODO: if-then with && + || ? How do we define the arguments and output?
            fn __eq__(&self, other: BoolLike) -> Self {
                let other = render_bool_like(other);
                Self(format!("({} == {})", self, other))
            }
            fn __ne__(&self, other: BoolLike) -> Self {
                let other = render_bool_like(other);
                Self(format!("({} != {})", self, other))
            }
            fn if_else(&self, condition: BoolLike, else_expr: BoolLike) -> BooleanExpression {
                let condition = render_bool_like(condition);
                let else_expr = render_bool_like(else_expr);
                BooleanExpression(format!("({} && {} || {})", condition, self, else_expr))
            }
            fn to_json(&self) -> ObjectExpression {
                ObjectExpression(format!("toJSON({})", self))
            }
            fn __str__(&self) -> String {
                self.as_expression_string()
            }
        }
        #[pyfunction]
        fn success() -> BooleanExpression {
            BooleanExpression("success()".to_string())
        }
        #[pyfunction]
        fn always() -> BooleanExpression {
            BooleanExpression("always()".to_string())
        }
        #[pyfunction]
        fn cancelled() -> BooleanExpression {
            BooleanExpression("cancelled()".to_string())
        }
        #[pyfunction]
        fn failure() -> BooleanExpression {
            BooleanExpression("failure()".to_string())
        }
        #[pyclass]
        #[derive(Clone)]
        pub struct NumberExpression(String);
        impl YamlExpression for NumberExpression {
            fn stringify(&self) -> &str {
                &self.0
            }
        }
        impl Display for NumberExpression {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.stringify())
            }
        }
        #[pymethods]
        impl NumberExpression {
            fn as_bool(&self) -> BooleanExpression {
                BooleanExpression(self.to_string())
            }
            fn as_str(&self) -> StringExpression {
                StringExpression(self.to_string())
            }
            fn as_obj(&self) -> ObjectExpression {
                ObjectExpression(self.to_string())
            }
            fn __lt__(&self, other: NumberLike) -> BooleanExpression {
                let other = render_number_like(other);
                BooleanExpression(format!("({} < {})", self, other))
            }
            fn __le__(&self, other: NumberLike) -> BooleanExpression {
                let other = render_number_like(other);
                BooleanExpression(format!("({} <= {})", self, other))
            }
            fn __gt__(&self, other: NumberLike) -> BooleanExpression {
                let other = render_number_like(other);
                BooleanExpression(format!("({} > {})", self, other))
            }
            fn __ge__(&self, other: NumberLike) -> BooleanExpression {
                let other = render_number_like(other);
                BooleanExpression(format!("({} >= {})", self, other))
            }
            fn __eq__(&self, other: NumberLike) -> BooleanExpression {
                let other = render_number_like(other);
                BooleanExpression(format!("({} == {})", self, other))
            }
            fn __ne__(&self, other: NumberLike) -> BooleanExpression {
                let other = render_number_like(other);
                BooleanExpression(format!("({} != {})", self, other))
            }
            fn if_else(&self, condition: BoolLike, else_expr: NumberLike) -> NumberExpression {
                let condition = render_bool_like(condition);
                let else_expr = render_number_like(else_expr);
                NumberExpression(format!("({} && {} || {})", condition, self, else_expr))
            }
            fn to_json(&self) -> ObjectExpression {
                ObjectExpression(format!("toJSON({})", self))
            }
            fn __str__(&self) -> String {
                self.as_expression_string()
            }
        }
        #[pyclass]
        #[derive(Clone)]
        pub struct StringExpression(String);
        impl YamlExpression for StringExpression {
            fn stringify(&self) -> &str {
                &self.0
            }
        }
        impl Display for StringExpression {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.stringify())
            }
        }
        #[pymethods]
        impl StringExpression {
            fn as_bool(&self) -> BooleanExpression {
                BooleanExpression(self.to_string())
            }
            fn as_num(&self) -> NumberExpression {
                NumberExpression(self.to_string())
            }
            fn as_obj(&self) -> ObjectExpression {
                ObjectExpression(self.to_string())
            }
            fn __eq__(&self, other: StringLike) -> BooleanExpression {
                let other = render_string_like(other);
                BooleanExpression(format!("({} == {})", self, other))
            }
            fn __ne__(&self, other: StringLike) -> BooleanExpression {
                let other = render_string_like(other);
                BooleanExpression(format!("({} != {})", self, other))
            }
            fn contains(&self, other: StringLike) -> BooleanExpression {
                let other = render_string_like(other);
                BooleanExpression(format!("contains({}, {})", self, other))
            }
            fn startswith(&self, other: StringLike) -> BooleanExpression {
                let other = render_string_like(other);
                BooleanExpression(format!("startsWith({}, {})", self, other))
            }
            fn endswith(&self, other: StringLike) -> BooleanExpression {
                let other = render_string_like(other);
                BooleanExpression(format!("endsWith({}, {})", self, other))
            }
            fn format(&self, args: Vec<StringLike>) -> StringExpression {
                StringExpression(format!(
                    "format({}, {})",
                    self,
                    args.into_iter()
                        .map(render_string_like)
                        .collect::<Vec<String>>()
                        .join(", ")
                ))
            }
            // I don't think we need join for single strings despite the docs
            fn to_json(&self) -> ObjectExpression {
                ObjectExpression(format!("toJSON({})", self))
            }
            fn from_json_to_bool(&self) -> BooleanExpression {
                BooleanExpression(format!("fromJSON({})", self))
            }
            fn from_json_to_num(&self) -> NumberExpression {
                NumberExpression(format!("fromJSON({})", self))
            }
            fn from_json_to_str(&self) -> Self {
                Self(format!("fromJSON({})", self))
            }
            fn from_json_to_array(&self) -> ArrayExpression {
                ArrayExpression(format!("fromJSON({})", self))
            }
            fn from_json_to_obj(&self) -> ObjectExpression {
                ObjectExpression(format!("fromJSON({})", self))
            }
            fn hash_files(&self, others: Option<Vec<StringLike>>) -> StringExpression {
                if let Some(others) = others {
                    StringExpression(format!(
                        "hashFiles({}, {})",
                        self,
                        others
                            .into_iter()
                            .map(render_string_like)
                            .collect::<Vec<String>>()
                            .join(", ")
                    ))
                } else {
                    StringExpression(format!("hashFiles({})", self))
                }
            }
            fn if_else(&self, condition: BoolLike, else_expr: StringLike) -> StringExpression {
                let condition = render_bool_like(condition);
                let else_expr = render_string_like(else_expr);
                StringExpression(format!("({} && {} || {})", condition, self, else_expr))
            }
            fn __str__(&self) -> String {
                self.as_expression_string()
            }
        }

        impl Display for ArrayExpression {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.stringify())
            }
        }

        #[pyclass]
        #[derive(Clone)]
        pub struct ArrayExpression(String);
        impl YamlExpression for ArrayExpression {
            fn stringify(&self) -> &str {
                &self.0
            }
        }
        #[pymethods]
        impl ArrayExpression {
            fn as_num(&self) -> NumberExpression {
                NumberExpression(self.to_string())
            }
            fn as_obj(&self) -> ObjectExpression {
                ObjectExpression(self.to_string())
            }
            fn contains(&self, other: ObjectExpression) -> BooleanExpression {
                BooleanExpression(format!("contains({}, {})", self, other.stringify()))
            }
            fn join(&self, separator: Option<StringLike>) -> StringExpression {
                if let Some(sep) = separator {
                    let sep = render_string_like(sep);
                    StringExpression(format!("join({}, {})", self, sep))
                } else {
                    StringExpression(format!("join({})", self))
                }
            }
            fn to_json(&self) -> ObjectExpression {
                ObjectExpression(format!("toJSON({})", self))
            }
            fn __str__(&self) -> String {
                self.as_expression_string()
            }
        }
        #[pyclass]
        #[derive(Clone)]
        pub struct ObjectExpression(String);
        impl YamlExpression for ObjectExpression {
            fn stringify(&self) -> &str {
                &self.0
            }
        }
        impl Display for ObjectExpression {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.stringify())
            }
        }
        impl ObjectExpression {
            fn format_access(base: &str, key: &str) -> String {
                if validate_string(key) {
                    format!("{base}.{key}")
                } else {
                    format!("{base}[{}]", escape_string(key))
                }
            }
        }
        #[pymethods]
        impl ObjectExpression {
            fn as_num(&self) -> NumberExpression {
                NumberExpression(self.stringify().to_string())
            }
            fn as_str(&self) -> StringExpression {
                StringExpression(self.stringify().to_string())
            }
            fn as_bool(&self) -> BooleanExpression {
                BooleanExpression(self.stringify().to_string())
            }
            fn as_array(&self) -> ArrayExpression {
                ArrayExpression(self.stringify().to_string())
            }
            fn to_json(&self) -> ObjectExpression {
                ObjectExpression(format!("toJSON({})", self.stringify()))
            }
            fn from_json_to_bool(&self) -> BooleanExpression {
                BooleanExpression(format!("fromJSON({})", self.stringify()))
            }
            fn from_json_to_num(&self) -> NumberExpression {
                NumberExpression(format!("fromJSON({})", self.stringify()))
            }
            fn from_json_to_str(&self) -> Self {
                Self(format!("fromJSON({})", self.stringify()))
            }
            fn from_json_to_array(&self) -> ArrayExpression {
                ArrayExpression(format!("fromJSON({})", self.stringify()))
            }
            fn from_json_to_obj(&self) -> ObjectExpression {
                ObjectExpression(format!("fromJSON({})", self.stringify()))
            }
            #[classattr]
            const __contains__: Option<Py<PyAny>> = None;
            fn __getitem__(&self, key: String) -> ObjectExpression {
                ObjectExpression(Self::format_access(self.stringify(), &key))
            }
            fn __getattr__(&self, key: String) -> ObjectExpression {
                self.__getitem__(key)
            }
            fn __str__(&self) -> String {
                self.as_expression_string()
            }
        }

        #[pyclass]
        pub struct GithubContext;
        #[pymethods]
        impl GithubContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression("github".to_string())
            }
            #[getter]
            fn action(&self) -> StringExpression {
                StringExpression("github.action".to_string())
            }
            #[getter]
            fn action_path(&self) -> StringExpression {
                StringExpression("github.action_path".to_string())
            }
            #[getter]
            fn action_ref(&self) -> StringExpression {
                StringExpression("github.action_ref".to_string())
            }
            #[getter]
            fn action_repository(&self) -> StringExpression {
                StringExpression("github.action_repository".to_string())
            }
            #[getter]
            fn action_status(&self) -> StringExpression {
                StringExpression("github.action_status".to_string())
            }
            #[getter]
            fn actor(&self) -> StringExpression {
                StringExpression("github.actor".to_string())
            }
            #[getter]
            fn actor_id(&self) -> StringExpression {
                StringExpression("github.actor_id".to_string())
            }
            #[getter]
            fn api_url(&self) -> StringExpression {
                StringExpression("github.api_url".to_string())
            }
            #[getter]
            fn base_ref(&self) -> StringExpression {
                StringExpression("github.base_ref".to_string())
            }
            #[getter]
            fn env(&self) -> StringExpression {
                StringExpression("github.env".to_string())
            }
            #[getter]
            fn event(&self) -> ObjectExpression {
                ObjectExpression("github.event".to_string())
            }
            #[getter]
            fn event_name(&self) -> StringExpression {
                StringExpression("github.event_name".to_string())
            }
            #[getter]
            fn event_path(&self) -> StringExpression {
                StringExpression("github.event_path".to_string())
            }
            #[getter]
            fn graphql_url(&self) -> StringExpression {
                StringExpression("github.graphql_url".to_string())
            }
            #[getter]
            fn head_ref(&self) -> StringExpression {
                StringExpression("github.head_ref".to_string())
            }
            #[getter]
            fn job(&self) -> StringExpression {
                StringExpression("github.job".to_string())
            }
            #[getter]
            fn path(&self) -> StringExpression {
                StringExpression("github.path".to_string())
            }
            #[getter]
            fn r#ref(&self) -> StringExpression {
                StringExpression("github.ref".to_string())
            }
            #[getter]
            fn ref_name(&self) -> StringExpression {
                StringExpression("github.ref_name".to_string())
            }
            #[getter]
            fn ref_protected(&self) -> BooleanExpression {
                BooleanExpression("github.ref_protected".to_string())
            }
            #[getter]
            fn ref_type(&self) -> StringExpression {
                StringExpression("github.ref_type".to_string())
            }
            #[getter]
            fn repository(&self) -> StringExpression {
                StringExpression("github.repository".to_string())
            }
            #[getter]
            fn reporitory_id(&self) -> StringExpression {
                StringExpression("github.reporitory_id".to_string())
            }
            #[getter]
            fn repositor_owner(&self) -> StringExpression {
                StringExpression("github.repositor_owner".to_string())
            }
            #[getter]
            fn repository_owner_id(&self) -> StringExpression {
                StringExpression("github.repository_owner_id".to_string())
            }
            #[getter]
            fn repository_url(&self) -> StringExpression {
                StringExpression("github.repositoryUrl".to_string())
            }
            #[getter]
            fn retention_days(&self) -> StringExpression {
                StringExpression("github.retention_days".to_string())
            }
            #[getter]
            fn run_id(&self) -> StringExpression {
                StringExpression("github.run_id".to_string())
            }
            #[getter]
            fn run_number(&self) -> StringExpression {
                StringExpression("github.run_number".to_string())
            }
            #[getter]
            fn run_attempt(&self) -> StringExpression {
                StringExpression("github.run_attempt".to_string())
            }
            #[getter]
            fn secret_source(&self) -> StringExpression {
                StringExpression("github.secret_source".to_string())
            }
            #[getter]
            fn server_url(&self) -> StringExpression {
                StringExpression("github.server_url".to_string())
            }
            #[getter]
            fn sha(&self) -> StringExpression {
                StringExpression("github.sha".to_string())
            }
            #[getter]
            fn token(&self) -> StringExpression {
                StringExpression("github.token".to_string())
            }
            #[getter]
            fn triggering_actor(&self) -> StringExpression {
                StringExpression("github.triggering_actor".to_string())
            }
            #[getter]
            fn workflow(&self) -> StringExpression {
                StringExpression("github.workflow".to_string())
            }
            #[getter]
            fn workflow_ref(&self) -> StringExpression {
                StringExpression("github.workflow_ref".to_string())
            }
            #[getter]
            fn workflow_sha(&self) -> StringExpression {
                StringExpression("github.workflow_sha".to_string())
            }
            #[getter]
            fn workspace(&self) -> StringExpression {
                StringExpression("github.workspace".to_string())
            }
        }

        #[pyclass]
        pub struct EnvContext;
        #[pymethods]
        impl EnvContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression("env".to_string())
            }
            #[classattr]
            const __contains__: Option<Py<PyAny>> = None;
            fn __getitem__(&self, key: String) -> StringExpression {
                StringExpression(ObjectExpression::format_access("env", &key))
            }
            fn __getattr__(&self, key: String) -> StringExpression {
                self.__getitem__(key)
            }
        }

        #[pyclass]
        pub struct VarsContext;
        #[pymethods]
        impl VarsContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression("vars".to_string())
            }
            #[classattr]
            const __contains__: Option<Py<PyAny>> = None;
            fn __getitem__(&self, key: String) -> StringExpression {
                StringExpression(ObjectExpression::format_access("vars", &key))
            }
            fn __getattr__(&self, key: String) -> StringExpression {
                self.__getitem__(key)
            }
        }

        #[pyclass]
        pub struct JobContainerContext;
        #[pymethods]
        impl JobContainerContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression("job.container".to_string())
            }
            #[getter]
            fn id(&self) -> StringExpression {
                StringExpression("job.container.id".to_string())
            }
            #[getter]
            fn network(&self) -> StringExpression {
                StringExpression("job.container.network".to_string())
            }
        }

        #[pyclass]
        pub struct JobServicesIdContext(String);
        #[pymethods]
        impl JobServicesIdContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression(self.0.clone())
            }
            #[getter]
            fn id(&self) -> StringExpression {
                StringExpression(format!("{}.id", self.0))
            }
            #[getter]
            fn network(&self) -> StringExpression {
                StringExpression(format!("{}.network", self.0))
            }
            #[getter]
            fn ports(&self) -> ObjectExpression {
                ObjectExpression(format!("{}.ports", self.0))
            }
        }

        #[pyclass]
        pub struct JobServicesContext;
        #[pymethods]
        impl JobServicesContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression("job.services".to_string())
            }
            #[classattr]
            const __contains__: Option<Py<PyAny>> = None;
            fn __getitem__(&self, key: String) -> JobServicesIdContext {
                JobServicesIdContext(ObjectExpression::format_access("job.services", &key))
            }
            fn __getattr__(&self, key: String) -> JobServicesIdContext {
                self.__getitem__(key)
            }
        }

        #[pyclass]
        pub struct JobContext;
        #[pymethods]
        impl JobContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression("job".to_string())
            }
            #[getter]
            fn check_run_id(&self) -> NumberExpression {
                NumberExpression("job.check_run_id".to_string())
            }
            #[getter]
            fn container(&self) -> JobContainerContext {
                JobContainerContext
            }
            #[getter]
            fn services(&self) -> JobServicesContext {
                JobServicesContext
            }
            #[getter]
            fn status(&self) -> StringExpression {
                StringExpression("job.status".to_string())
            }
        }

        #[pyclass]
        pub struct JobsJobIdOutputsContext(String);
        #[pymethods]
        impl JobsJobIdOutputsContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression(self.0.clone())
            }
            #[classattr]
            const __contains__: Option<Py<PyAny>> = None;
            fn __getitem__(&self, key: String) -> StringExpression {
                StringExpression(ObjectExpression::format_access(&self.0, &key))
            }
            fn __getattr__(&self, key: String) -> StringExpression {
                self.__getitem__(key)
            }
        }

        #[pyclass]
        pub struct JobsJobIdContext(String);
        #[pymethods]
        impl JobsJobIdContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression(self.0.clone())
            }
            #[getter]
            fn result(&self) -> StringExpression {
                StringExpression(format!("{}.result", self.0))
            }
            #[getter]
            fn outputs(&self) -> JobsJobIdOutputsContext {
                JobsJobIdOutputsContext(format!("{}.outputs", self.0))
            }
        }

        #[pyclass]
        pub struct JobsContext;
        #[pymethods]
        impl JobsContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression("jobs".to_string())
            }
            #[classattr]
            const __contains__: Option<Py<PyAny>> = None;
            fn __getitem__(&self, key: String) -> JobsJobIdContext {
                JobsJobIdContext(ObjectExpression::format_access("jobs", &key))
            }
            fn __getattr__(&self, key: String) -> JobsJobIdContext {
                self.__getitem__(key)
            }
        }

        #[pyclass]
        pub struct StepsStepIdOutputsContext(String);
        #[pymethods]
        impl StepsStepIdOutputsContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression(self.0.clone())
            }
            #[classattr]
            const __contains__: Option<Py<PyAny>> = None;
            fn __getitem__(&self, key: String) -> StringExpression {
                StringExpression(ObjectExpression::format_access(&self.0, &key))
            }
            fn __getattr__(&self, key: String) -> StringExpression {
                self.__getitem__(key)
            }
        }

        #[pyclass]
        pub struct StepsStepIdContext(String);
        #[pymethods]
        impl StepsStepIdContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression(self.0.clone())
            }
            #[getter]
            fn outputs(&self) -> StepsStepIdOutputsContext {
                StepsStepIdOutputsContext(format!("{}.outputs", self.0))
            }
            #[getter]
            fn conclusion(&self) -> StringExpression {
                StringExpression(format!("{}.conclusion", self.0))
            }
            #[getter]
            fn outcome(&self) -> StringExpression {
                StringExpression(format!("{}.outcome", self.0))
            }
        }

        #[pyclass]
        pub struct StepsContext;
        #[pymethods]
        impl StepsContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression("steps".to_string())
            }
            #[classattr]
            const __contains__: Option<Py<PyAny>> = None;
            fn __getitem__(&self, key: String) -> StepsStepIdContext {
                StepsStepIdContext(ObjectExpression::format_access("steps", &key))
            }
            fn __getattr__(&self, key: String) -> StepsStepIdContext {
                self.__getitem__(key)
            }
        }

        #[pyclass]
        pub struct RunnerContext;
        #[pymethods]
        impl RunnerContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression("runner".to_string())
            }
            #[getter]
            fn name(&self) -> StringExpression {
                StringExpression("runner.name".to_string())
            }
            #[getter]
            fn os(&self) -> StringExpression {
                StringExpression("runner.os".to_string())
            }
            #[getter]
            fn arch(&self) -> StringExpression {
                StringExpression("runner.arch".to_string())
            }
            #[getter]
            fn temp(&self) -> StringExpression {
                StringExpression("runner.temp".to_string())
            }
            #[getter]
            fn tool_cache(&self) -> StringExpression {
                StringExpression("runner.tool_cache".to_string())
            }
            #[getter]
            fn debug(&self) -> StringExpression {
                StringExpression("runner.debug".to_string())
            }
            #[getter]
            fn environment(&self) -> StringExpression {
                StringExpression("runner.environment".to_string())
            }
        }

        #[pyclass]
        pub struct SecretsContext;
        #[pymethods]
        impl SecretsContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression("secrets".to_string())
            }
            #[getter]
            fn github_token(&self) -> StringExpression {
                StringExpression("secrets.GITHUB_TOKEN".to_string())
            }
            #[classattr]
            const __contains__: Option<Py<PyAny>> = None;
            fn __getitem__(&self, key: String) -> StringExpression {
                StringExpression(ObjectExpression::format_access("secrets", &key))
            }
            fn __getattr__(&self, key: String) -> StringExpression {
                self.__getitem__(key)
            }
        }

        #[pyclass]
        pub struct StrategyContext;
        #[pymethods]
        impl StrategyContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression("strategy".to_string())
            }
            #[getter]
            fn fail_fast(&self) -> BooleanExpression {
                BooleanExpression("strategy.fail-fast".to_string())
            }
            #[getter]
            fn job_index(&self) -> NumberExpression {
                NumberExpression("strategy.job-index".to_string())
            }
            #[getter]
            fn job_total(&self) -> NumberExpression {
                NumberExpression("strategy.job-total".to_string())
            }
            #[getter]
            fn max_parallel(&self) -> NumberExpression {
                NumberExpression("strategy.max-parallel".to_string())
            }
        }

        #[pyclass]
        pub struct MatrixContext;
        #[pymethods]
        impl MatrixContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression("matrix".to_string())
            }
            #[classattr]
            const __contains__: Option<Py<PyAny>> = None;
            fn __getitem__(&self, key: String) -> ObjectExpression {
                ObjectExpression(ObjectExpression::format_access("matrix", &key))
            }
            fn __getattr__(&self, key: String) -> ObjectExpression {
                self.__getitem__(key)
            }
        }

        #[pyclass]
        pub struct NeedsJobIdOutputsContext(String);
        #[pymethods]
        impl NeedsJobIdOutputsContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression(self.0.clone())
            }
            #[classattr]
            const __contains__: Option<Py<PyAny>> = None;
            fn __getitem__(&self, key: String) -> StringExpression {
                StringExpression(ObjectExpression::format_access(&self.0, &key))
            }
            fn __getattr__(&self, key: String) -> StringExpression {
                self.__getitem__(key)
            }
        }

        #[pyclass]
        pub struct NeedsJobIdContext(String);
        #[pymethods]
        impl NeedsJobIdContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression(self.0.clone())
            }
            #[getter]
            fn outputs(&self) -> NeedsJobIdOutputsContext {
                NeedsJobIdOutputsContext(format!("{}.outputs", self.0))
            }
            #[getter]
            fn result(&self) -> StringExpression {
                StringExpression(format!("{}.result", self.0))
            }
        }

        #[pyclass]
        pub struct NeedsContext;
        #[pymethods]
        impl NeedsContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression("needs".to_string())
            }
            #[classattr]
            const __contains__: Option<Py<PyAny>> = None;
            fn __getitem__(&self, key: String) -> NeedsJobIdContext {
                NeedsJobIdContext(ObjectExpression::format_access("needs", &key))
            }
            fn __getattr__(&self, key: String) -> NeedsJobIdContext {
                self.__getitem__(key)
            }
        }

        #[pyclass]
        pub struct InputsContext;
        #[pymethods]
        impl InputsContext {
            #[getter]
            fn expr(&self) -> ObjectExpression {
                ObjectExpression("inputs".to_string())
            }
            #[classattr]
            const __contains__: Option<Py<PyAny>> = None;
            fn __getitem__(&self, key: String) -> ObjectExpression {
                ObjectExpression(ObjectExpression::format_access("inputs", &key))
            }
            fn __getattr__(&self, key: String) -> ObjectExpression {
                self.__getitem__(key)
            }
        }

        #[pyclass(name = "context")]
        pub struct Context;
        #[allow(non_upper_case_globals)]
        #[pymethods]
        impl Context {
            #[classattr]
            const github: GithubContext = GithubContext;
            #[classattr]
            const env: EnvContext = EnvContext;
            #[classattr]
            const vars: VarsContext = VarsContext;
            #[classattr]
            const job: JobContext = JobContext;
            #[classattr]
            const jobs: JobsContext = JobsContext;
            #[classattr]
            const steps: StepsContext = StepsContext;
            #[classattr]
            const runner: RunnerContext = RunnerContext;
            #[classattr]
            const secrets: SecretsContext = SecretsContext;
            #[classattr]
            const strategy: StrategyContext = StrategyContext;
            #[classattr]
            const matrix: MatrixContext = MatrixContext;
            #[classattr]
            const needs: NeedsContext = NeedsContext;
            #[classattr]
            const inputs: InputsContext = InputsContext;
        }

        // TODO: Does toJSON return a string?

        fn escape_string(s: &str) -> String {
            let mut out = String::with_capacity(s.len() + 2);
            out.push('\'');
            for ch in s.chars() {
                if ch == '\'' {
                    out.push_str("''");
                } else if !push_escaped_control(&mut out, ch) {
                    out.push(ch);
                }
            }
            out.push('\'');
            out
        }

        fn validate_string(s: &str) -> bool {
            let mut chars = s.chars();
            match chars.next() {
                Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
                _ => return false,
            }
            chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        }

        #[pyfunction]
        fn lit_str(s: String) -> StringExpression {
            StringExpression(escape_string(&s))
        }

        #[pyfunction]
        fn lit_bool(b: bool) -> BooleanExpression {
            BooleanExpression(if b {
                "true".to_string()
            } else {
                "false".to_string()
            })
        }

        #[pyfunction]
        fn lit_num(n: Bound<PyAny>) -> PyResult<NumberExpression> {
            if n.is_instance_of::<PyFloat>() {
                Ok(NumberExpression(n.extract::<f64>()?.to_string()))
            } else if n.is_instance_of::<PyInt>() {
                Ok(NumberExpression(n.extract::<i64>()?.to_string()))
            } else {
                Err(PyValueError::new_err("Expected a number"))
            }
        }
    }

    type StringLike = Either<StringExpression, String>;
    type BoolLike = Either<BooleanExpression, bool>;
    type IntLike = Either<NumberExpression, i64>;

    impl TryYamlable for Bound<'_, PyAny> {
        fn try_as_yaml(&self) -> PyResult<Yaml> {
            if self.is_none() {
                Ok(Yaml::Null)
            } else if let Ok(e) = self.extract::<StringExpression>() {
                Ok((&e).as_yaml())
            } else if let Ok(e) = self.extract::<BooleanExpression>() {
                Ok((&e).as_yaml())
            } else if let Ok(e) = self.extract::<NumberExpression>() {
                Ok((&e).as_yaml())
            } else if self.is_instance_of::<PyBool>() {
                Ok(self.extract::<bool>()?.as_yaml())
            } else if self.is_instance_of::<PyInt>() {
                Ok(self.extract::<i64>()?.as_yaml())
            } else if self.is_instance_of::<PyFloat>() {
                Ok(self.extract::<f64>()?.as_yaml())
            } else if self.is_instance_of::<PyString>() {
                Ok(self.extract::<String>()?.as_yaml())
            } else if let Ok(list) = self.cast::<PyList>() {
                Ok(Yaml::Array(list.try_as_array()?))
            } else if let Ok(dict) = self.cast::<PyDict>() {
                Ok(Yaml::Hash(dict.try_as_hash()?))
            } else {
                Err(PyValueError::new_err("Invalid value"))
            }
        }
    }

    impl TryHash for Bound<'_, PyDict> {
        fn try_as_hash(&self) -> PyResult<Hash> {
            let mut dict_internals = Hash::new();
            for (key, entry) in self.iter() {
                if let Ok(key) = key.extract::<String>() {
                    dict_internals.insert_yaml(key, entry.try_as_yaml()?)
                } else {
                    return Err(PyValueError::new_err("Invalid key"));
                }
            }
            Ok(dict_internals)
        }
    }

    impl TryArray for Bound<'_, PyList> {
        fn try_as_array(&self) -> PyResult<Vec<Yaml>> {
            let mut list_internals = Vec::new();
            for entry in self.iter() {
                list_internals.push(entry.try_as_yaml()?)
            }
            Ok(list_internals)
        }
    }

    #[derive(Clone)]
    struct WithArgs {
        options: Option<Hash>,
        args: Option<StringLike>,
        entrypoint: Option<StringLike>,
    }

    impl Yamlable for WithArgs {
        fn as_yaml(&self) -> Yaml {
            let mut entries = self.options.clone().unwrap_or_default();
            entries.insert_yaml_opt("args", &self.args);
            entries.insert_yaml_opt("entrypoint", &self.entrypoint);
            Yaml::Hash(entries)
        }
    }

    #[derive(Clone)]
    enum StepAction {
        Run(StringLike),
        Action {
            uses: String,
            with: Option<WithArgs>,
        },
    }
    impl StepAction {
        fn uses(&self) -> Option<String> {
            match self {
                StepAction::Run(_) => None,
                StepAction::Action { uses, .. } => Some(uses.to_string()),
            }
        }
        fn with(&self) -> Option<WithArgs> {
            match self {
                StepAction::Run(_) => None,
                StepAction::Action { with, .. } => with.clone(),
            }
        }
        fn run(&self) -> Option<&StringLike> {
            match self {
                StepAction::Run(script) => Some(script),
                StepAction::Action { .. } => None,
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct Step {
        name: StringLike,
        step_action: StepAction,
        options: StepOptions,
    }

    #[derive(Clone)]
    struct StepOptions {
        condition: Option<Either<BooleanExpression, String>>,
        working_directory: Option<StringLike>,
        shell: Option<String>,
        id: Option<String>,
        env: Option<PyMap<String, StringLike>>,
        continue_on_error: Option<BoolLike>,
        timeout_minutes: Option<IntLike>,
    }

    #[pymethods]
    impl Step {
        fn __str__(&self) -> PyResult<String> {
            self.as_yaml_string()
        }
    }
    impl Yamlable for Step {
        fn as_yaml(&self) -> Yaml {
            let mut entries = Hash::new();
            entries.insert_yaml("name", &self.name);
            entries.insert_yaml_opt("if", &self.options.condition);
            entries.insert_yaml_opt("uses", self.step_action.uses());
            entries.insert_yaml_opt("with", self.step_action.with());
            entries.insert_yaml_opt("run", self.step_action.run());
            entries.insert_yaml_opt("working-directory", &self.options.working_directory);
            entries.insert_yaml_opt("shell", &self.options.shell);
            entries.insert_yaml_opt("id", &self.options.id);
            entries.insert_yaml_opt("env", &self.options.env);
            entries.insert_yaml_opt("continue-on-error", &self.options.continue_on_error);
            entries.insert_yaml_opt("timeout-minutes", &self.options.timeout_minutes);
            Yaml::Hash(entries)
        }
    }
    fn collect_script_lines(script: Vec<StringLike>) -> StringLike {
        let lines = script
            .into_iter()
            .map(|line| match line {
                Either::A(expr) => expr.as_expression_string(),
                Either::B(raw) => raw,
            })
            .collect::<Vec<String>>()
            .join("\n");
        Either::B(lines)
    }

    #[pyfunction]
    #[pyo3(signature = (name, *script, condition = None, working_directory = None, shell = None, id = None, env = None, continue_on_error = None, timeout_minutes= None))]
    fn script(
        name: StringLike,
        script: &Bound<'_, PyTuple>,
        condition: Option<Either<BooleanExpression, String>>,
        working_directory: Option<StringLike>,
        shell: Option<String>,
        id: Option<String>,
        env: Option<PyMap<String, StringLike>>,
        continue_on_error: Option<BoolLike>,
        timeout_minutes: Option<IntLike>,
    ) -> PyResult<Step> {
        let script = collect_script_lines(
            script
                .iter()
                .map(|item| item.extract::<StringLike>())
                .collect::<PyResult<Vec<StringLike>>>()?,
        );
        Ok(Step {
            name,
            step_action: StepAction::Run(script),
            options: StepOptions {
                condition,
                working_directory,
                shell,
                id,
                env,
                continue_on_error,
                timeout_minutes,
            },
        })
    }
    fn make_action(
        name: StringLike,
        action: String,
        r#ref: Option<String>,
        with_opts: Option<Hash>,
        args: Option<StringLike>,
        entrypoint: Option<StringLike>,
        condition: Option<Either<BooleanExpression, String>>,
        working_directory: Option<StringLike>,
        shell: Option<String>,
        id: Option<String>,
        env: Option<PyMap<String, StringLike>>,
        continue_on_error: Option<BoolLike>,
        timeout_minutes: Option<IntLike>,
    ) -> PyResult<Step> {
        let with_args = if with_opts.is_some() || args.is_some() || entrypoint.is_some() {
            Some(WithArgs {
                options: with_opts,
                args,
                entrypoint,
            })
        } else {
            None
        };
        Ok(Step {
            name,
            step_action: StepAction::Action {
                uses: format!(
                    "{}{}",
                    action,
                    r#ref.map(|s| format!("@{}", s)).unwrap_or_default()
                ),
                with: with_args,
            },
            options: StepOptions {
                condition,
                working_directory,
                shell,
                id,
                env,
                continue_on_error,
                timeout_minutes,
            },
        })
    }
    #[pyfunction]
    #[pyo3(signature = (name, action, *, r#ref = None, with_opts = None, args = None, entrypoint = None, condition = None, working_directory = None, shell = None, id = None, env = None, continue_on_error = None, timeout_minutes = None))]
    fn action(
        name: StringLike,
        action: String,
        r#ref: Option<String>,
        with_opts: Option<Bound<PyDict>>,
        args: Option<StringLike>,
        entrypoint: Option<StringLike>,
        condition: Option<Either<BooleanExpression, String>>,
        working_directory: Option<StringLike>,
        shell: Option<String>,
        id: Option<String>,
        env: Option<PyMap<String, StringLike>>,
        continue_on_error: Option<BoolLike>,
        timeout_minutes: Option<IntLike>,
    ) -> PyResult<Step> {
        make_action(
            name,
            action,
            r#ref,
            with_opts.map(|d| d.try_as_hash()).transpose()?,
            args,
            entrypoint,
            condition,
            working_directory,
            shell,
            id,
            env,
            continue_on_error,
            timeout_minutes,
        )
    }

    #[derive(Clone, Copy)]
    enum ReadWriteNonePermission {
        Read,
        Write,
        None,
    }
    impl FromStr for ReadWriteNonePermission {
        type Err = PyErr;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "read" => Ok(Self::Read),
                "write" => Ok(Self::Write),
                "none" => Ok(Self::None),
                _ => Err(PyValueError::new_err("Invalid permission")),
            }
        }
    }
    impl Yamlable for &ReadWriteNonePermission {
        fn as_yaml(&self) -> Yaml {
            match self {
                ReadWriteNonePermission::Read => "read",
                ReadWriteNonePermission::Write => "write",
                ReadWriteNonePermission::None => "none",
            }
            .as_yaml()
        }
    }
    #[derive(Clone, Copy)]
    enum WriteNonePermission {
        Write,
        None,
    }
    impl FromStr for WriteNonePermission {
        type Err = PyErr;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "write" => Ok(Self::Write),
                "none" => Ok(Self::None),
                _ => Err(PyValueError::new_err("Invalid permission")),
            }
        }
    }
    impl Yamlable for &WriteNonePermission {
        fn as_yaml(&self) -> Yaml {
            match self {
                WriteNonePermission::Write => "write",
                WriteNonePermission::None => "none",
            }
            .as_yaml()
        }
    }
    #[derive(Clone, Copy)]
    enum ReadNonePermission {
        Read,
        None,
    }
    impl FromStr for ReadNonePermission {
        type Err = PyErr;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "read" => Ok(Self::Read),
                "none" => Ok(Self::None),
                _ => Err(PyValueError::new_err("Invalid permission")),
            }
        }
    }
    impl Yamlable for &ReadNonePermission {
        fn as_yaml(&self) -> Yaml {
            match self {
                ReadNonePermission::Read => "read",
                ReadNonePermission::None => "none",
            }
            .as_yaml()
        }
    }

    #[derive(Clone)]
    struct IndividualPermissions {
        actions: Option<ReadWriteNonePermission>,
        artifact_metadata: Option<ReadWriteNonePermission>,
        attestations: Option<ReadWriteNonePermission>,
        checks: Option<ReadWriteNonePermission>,
        contents: Option<ReadWriteNonePermission>,
        deployments: Option<ReadWriteNonePermission>,
        id_token: Option<WriteNonePermission>,
        issues: Option<ReadWriteNonePermission>,
        models: Option<ReadNonePermission>,
        discussions: Option<ReadWriteNonePermission>,
        packages: Option<ReadWriteNonePermission>,
        pages: Option<ReadWriteNonePermission>,
        pull_requests: Option<ReadWriteNonePermission>,
        security_events: Option<ReadWriteNonePermission>,
        statuses: Option<ReadWriteNonePermission>,
    }
    #[derive(Clone)]
    enum PermissionsOptions {
        Individual(IndividualPermissions),
        ReadAll,
        WriteAll,
        None,
    }
    #[pyclass]
    #[derive(Clone)]
    struct Permissions {
        options: PermissionsOptions,
    }
    #[pymethods]
    impl Permissions {
        #[new]
        #[pyo3(signature= (actions=None, artifact_metadata=None, attestations=None, checks=None, contents=None, deployments=None, id_token=None, issues=None, models=None, discussions=None, packages=None, pages=None, pull_requests=None, security_events=None, statuses=None))]
        fn new(
            actions: Option<String>,
            artifact_metadata: Option<String>,
            attestations: Option<String>,
            checks: Option<String>,
            contents: Option<String>,
            deployments: Option<String>,
            id_token: Option<String>,
            issues: Option<String>,
            models: Option<String>,
            discussions: Option<String>,
            packages: Option<String>,
            pages: Option<String>,
            pull_requests: Option<String>,
            security_events: Option<String>,
            statuses: Option<String>,
        ) -> PyResult<Self> {
            Ok(Self {
                options: PermissionsOptions::Individual(IndividualPermissions {
                    actions: actions.map(|s| s.parse()).transpose()?,
                    artifact_metadata: artifact_metadata.map(|s| s.parse()).transpose()?,
                    attestations: attestations.map(|s| s.parse()).transpose()?,
                    checks: checks.map(|s| s.parse()).transpose()?,
                    contents: contents.map(|s| s.parse()).transpose()?,
                    deployments: deployments.map(|s| s.parse()).transpose()?,
                    id_token: id_token.map(|s| s.parse()).transpose()?,
                    issues: issues.map(|s| s.parse()).transpose()?,
                    models: models.map(|s| s.parse()).transpose()?,
                    discussions: discussions.map(|s| s.parse()).transpose()?,
                    packages: packages.map(|s| s.parse()).transpose()?,
                    pages: pages.map(|s| s.parse()).transpose()?,
                    pull_requests: pull_requests.map(|s| s.parse()).transpose()?,
                    security_events: security_events.map(|s| s.parse()).transpose()?,
                    statuses: statuses.map(|s| s.parse()).transpose()?,
                }),
            })
        }
        #[staticmethod]
        fn none() -> Self {
            Self {
                options: PermissionsOptions::None,
            }
        }
        #[staticmethod]
        fn read_all() -> Self {
            Self {
                options: PermissionsOptions::ReadAll,
            }
        }
        #[staticmethod]
        fn write_all() -> Self {
            Self {
                options: PermissionsOptions::WriteAll,
            }
        }
        fn __str__(&self) -> PyResult<String> {
            self.as_yaml_string()
        }
    }
    impl Yamlable for &Permissions {
        fn as_yaml(&self) -> Yaml {
            match &self.options {
                PermissionsOptions::Individual(indiv_perms) => {
                    let mut permissions = Hash::new();
                    permissions.insert_yaml_opt("actions", &indiv_perms.actions);
                    permissions
                        .insert_yaml_opt("artifact-metadata", &indiv_perms.artifact_metadata);
                    permissions.insert_yaml_opt("attestations", &indiv_perms.attestations);
                    permissions.insert_yaml_opt("checks", &indiv_perms.checks);
                    permissions.insert_yaml_opt("contents", &indiv_perms.contents);
                    permissions.insert_yaml_opt("deployments", &indiv_perms.deployments);
                    permissions.insert_yaml_opt("id-token", &indiv_perms.id_token);
                    permissions.insert_yaml_opt("issues", &indiv_perms.issues);
                    permissions.insert_yaml_opt("models", &indiv_perms.models);
                    permissions.insert_yaml_opt("discussion", &indiv_perms.discussions);
                    permissions.insert_yaml_opt("packages", &indiv_perms.packages);
                    permissions.insert_yaml_opt("pages", &indiv_perms.pages);
                    permissions.insert_yaml_opt("pull-requests", &indiv_perms.pull_requests);
                    permissions.insert_yaml_opt("security-events", &indiv_perms.security_events);
                    permissions.insert_yaml_opt("statuses", &indiv_perms.statuses);
                    Yaml::Hash(permissions)
                }
                PermissionsOptions::ReadAll => "read-all".as_yaml(),
                PermissionsOptions::WriteAll => "write-all".as_yaml(),
                PermissionsOptions::None => Yaml::Hash(Hash::new()), // TODO: test
            }
        }
    }

    #[derive(Clone)]
    enum RunsOnSpecOptions {
        Group(StringLike),
        Labels(StringLike),
        GroupAndLabels(StringLike, StringLike),
    }
    #[pyclass]
    #[derive(Clone)]
    struct RunsOnSpec {
        options: RunsOnSpecOptions,
    }
    #[pymethods]
    impl RunsOnSpec {
        #[new]
        fn new(group: StringLike, labels: StringLike) -> Self {
            Self {
                options: RunsOnSpecOptions::GroupAndLabels(group, labels),
            }
        }
        #[staticmethod]
        fn group(group: StringLike) -> Self {
            Self {
                options: RunsOnSpecOptions::Group(group),
            }
        }
        #[staticmethod]
        fn labels(labels: StringLike) -> Self {
            Self {
                options: RunsOnSpecOptions::Labels(labels),
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.as_yaml_string()
        }
    }
    impl Yamlable for &RunsOnSpec {
        fn as_yaml(&self) -> Yaml {
            let mut out = Hash::new();
            match &self.options {
                RunsOnSpecOptions::Group(group) => out.insert_yaml("group", group),
                RunsOnSpecOptions::Labels(labels) => out.insert_yaml("labels", labels),
                RunsOnSpecOptions::GroupAndLabels(group, labels) => {
                    out.insert_yaml("group", group);
                    out.insert_yaml("labels", labels);
                }
            }
            Yaml::Hash(out)
        }
    }

    #[derive(Clone)]
    enum RunsOn {
        String(StringLike),
        Array(Vec<StringLike>),
        Spec(RunsOnSpec),
    }
    impl<'a, 'py> FromPyObject<'a, 'py> for RunsOn {
        type Error = PyErr;

        fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
            if let Ok(spec) = obj.extract::<RunsOnSpec>() {
                Ok(Self::Spec(spec))
            } else if let Ok(list) = obj.extract::<Vec<StringLike>>() {
                Ok(Self::Array(list))
            } else if let Ok(single) = obj.extract::<StringLike>() {
                Ok(Self::String(single))
            } else {
                Err(PyValueError::new_err(
                    "Expected a 'RunsOnSpec', list of strings, or a single string",
                ))
            }
        }
    }
    impl Yamlable for &RunsOn {
        fn as_yaml(&self) -> Yaml {
            match self {
                RunsOn::String(s) => s.as_yaml(),
                RunsOn::Array(l) => l.as_yaml(),
                RunsOn::Spec(spec) => spec.as_yaml(),
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct Environment {
        name: StringLike,
        url: Option<StringLike>,
    }
    #[pymethods]
    impl Environment {
        #[new]
        #[pyo3(signature = (name, url = None))]
        fn new(name: StringLike, url: Option<StringLike>) -> Self {
            Self { name, url }
        }

        fn __str__(&self) -> PyResult<String> {
            self.as_yaml_string()
        }
    }
    impl Yamlable for &Environment {
        fn as_yaml(&self) -> Yaml {
            if let Some(url) = &self.url {
                let mut sub = Hash::new();
                sub.insert_yaml("name", &self.name);
                sub.insert_yaml("url", url);
                Yaml::Hash(sub)
            } else {
                self.name.as_yaml()
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct Concurrency {
        group: StringLike,
        cancel_in_progress: Option<BoolLike>,
    }
    #[pymethods]
    impl Concurrency {
        #[new]
        #[pyo3(signature = (group, *, cancel_in_progress=None))]
        fn new(group: StringLike, cancel_in_progress: Option<BoolLike>) -> Self {
            Self {
                group,
                cancel_in_progress,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.as_yaml_string()
        }
    }
    impl Yamlable for &Concurrency {
        fn as_yaml(&self) -> Yaml {
            let mut out = Hash::new();
            out.insert_yaml("group", &self.group);
            out.insert_yaml_opt("cancel-in-progress", &self.cancel_in_progress);
            Yaml::Hash(out)
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct RunDefaults {
        shell: Option<StringLike>,
        working_directory: Option<StringLike>,
    }
    #[pymethods]
    impl RunDefaults {
        #[new]
        #[pyo3(signature = (*, shell=None, working_directory=None))]
        fn new(shell: Option<StringLike>, working_directory: Option<StringLike>) -> Self {
            Self {
                shell,
                working_directory,
            }
        }
    }
    impl MaybeYamlable for &RunDefaults {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            let mut out = Hash::new();
            out.insert_yaml_opt("shell", &self.shell);
            out.insert_yaml_opt("working-directory", &self.working_directory);
            if out.is_empty() {
                None
            } else {
                Some(Yaml::Hash(out))
            }
        }
    }
    #[pyclass]
    #[derive(Clone)]
    struct Defaults {
        defaults: Option<PyMap<String, String>>,
        run_defaults: Option<RunDefaults>,
    }
    #[pymethods]
    impl Defaults {
        #[new]
        #[pyo3(signature = (*, defaults=None, run_defaults=None))]
        fn new(defaults: Option<PyMap<String, String>>, run_defaults: Option<RunDefaults>) -> Self {
            Self {
                defaults,
                run_defaults,
            }
        }
    }
    impl MaybeYamlable for &Defaults {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            let mut out = Hash::new();
            if let Some(run_defaults) = &self.run_defaults {
                out.insert_yaml_opt("run", run_defaults.maybe_as_yaml());
            }
            out.insert_yaml_opt("defaults", &self.defaults);
            if out.is_empty() {
                None
            } else {
                Some(Yaml::Hash(out))
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct Matrix {
        matrix: Option<Hash>,
        include: Option<Array>,
        exclude: Option<Array>,
    }
    #[pymethods]
    impl Matrix {
        #[new]
        #[pyo3(signature = (*, include = None, exclude = None, **matrix))]
        fn new(
            include: Option<&Bound<'_, PyList>>,
            exclude: Option<&Bound<'_, PyList>>,
            matrix: Option<&Bound<'_, PyDict>>,
        ) -> PyResult<Self> {
            Ok(Self {
                matrix: matrix
                    .map(|m| {
                        let mut hash = Hash::new();
                        for (k, v) in m.iter() {
                            hash.insert_yaml(k.try_as_yaml()?, v.try_as_yaml()?)
                        }
                        Ok::<Hash, PyErr>(hash)
                    })
                    .transpose()?,
                include: include
                    .map(|i| {
                        let mut arr = Array::new();
                        for v in i.iter() {
                            arr.push_yaml(v.try_as_yaml()?)
                        }
                        Ok::<Array, PyErr>(arr)
                    })
                    .transpose()?,
                exclude: exclude
                    .map(|e| {
                        let mut arr = Array::new();
                        for v in e.iter() {
                            arr.push_yaml(v.try_as_yaml()?)
                        }
                        Ok::<Array, PyErr>(arr)
                    })
                    .transpose()?,
            })
        }

        fn __str__(&self) -> PyResult<String> {
            self.as_yaml_string()
        }
    }
    impl Yamlable for &Matrix {
        fn as_yaml(&self) -> Yaml {
            let mut matrix = self.matrix.clone().unwrap_or_default();
            matrix.insert_yaml_opt("include", &self.include);
            matrix.insert_yaml_opt("exclude", &self.exclude);
            Yaml::Hash(matrix)
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct Strategy {
        matrix: Option<Matrix>,
        fast_fail: Option<BoolLike>,
        max_parallel: Option<IntLike>,
    }
    #[pymethods]
    impl Strategy {
        #[new]
        #[pyo3(signature = (*, matrix = None, fast_fail = None, max_parallel = None))]
        fn new(
            // TODO: prevent invalid state where all are None
            matrix: Option<Matrix>,
            fast_fail: Option<BoolLike>,
            max_parallel: Option<IntLike>,
        ) -> Self {
            Self {
                matrix,
                fast_fail,
                max_parallel,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.as_yaml_string()
        }
    }
    impl Yamlable for &Strategy {
        fn as_yaml(&self) -> Yaml {
            let mut strategy = Hash::new();
            strategy.insert_yaml_opt("matrix", &self.matrix);
            strategy.insert_yaml_opt("fail-fast", &self.fast_fail);
            strategy.insert_yaml_opt("max-parallel", &self.max_parallel);
            Yaml::Hash(strategy)
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct Credentials {
        username: StringLike,
        password: StringLike,
    }
    #[pymethods]
    impl Credentials {
        #[new]
        fn new(username: StringLike, password: StringLike) -> Self {
            Self { username, password }
        }

        fn __str__(&self) -> PyResult<String> {
            self.as_yaml_string()
        }
    }
    impl Yamlable for &Credentials {
        fn as_yaml(&self) -> Yaml {
            let mut out = Hash::new();
            out.insert_yaml("username", &self.username);
            out.insert_yaml("password", &self.password);
            Yaml::Hash(out)
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct Container {
        image: StringLike,
        credentials: Option<Credentials>,
        env: Option<PyMap<String, StringLike>>,
        ports: Option<Vec<IntLike>>,
        volumes: Option<Vec<StringLike>>,
        options: Option<StringLike>,
    }
    #[pymethods]
    impl Container {
        #[new]
        #[pyo3(signature = (image, *, credentials = None, env = None, ports = None, volumes = None, options = None))]
        fn new(
            image: StringLike,
            credentials: Option<Credentials>,
            env: Option<PyMap<String, StringLike>>,
            ports: Option<Vec<IntLike>>,
            volumes: Option<Vec<StringLike>>,
            options: Option<StringLike>,
        ) -> Self {
            Self {
                image,
                credentials,
                env,
                ports,
                volumes,
                options,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.as_yaml_string()
        }
    }
    impl Yamlable for &Container {
        fn as_yaml(&self) -> Yaml {
            let mut out = Hash::new();
            out.insert_yaml("image", &self.image);
            out.insert_yaml_opt("credentials", &self.credentials);
            out.insert_yaml_opt("env", &self.env);
            out.insert_yaml_opt("ports", &self.ports);
            out.insert_yaml_opt("volumes", &self.volumes);
            out.insert_yaml_opt("options", &self.options);
            Yaml::Hash(out)
        }
    }

    #[derive(Clone)]
    enum JobSecretsOptions {
        Secrets(HashMap<String, StringLike>),
        Inherit,
    }
    #[pyclass]
    #[derive(Clone)]
    struct JobSecrets {
        options: JobSecretsOptions,
    }
    #[pymethods]
    impl JobSecrets {
        #[new]
        fn new(secrets: HashMap<String, StringLike>) -> Self {
            Self {
                options: JobSecretsOptions::Secrets(secrets),
            }
        }
        #[staticmethod]
        fn inherit() -> Self {
            Self {
                options: JobSecretsOptions::Inherit,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.as_yaml_string()
        }
    }
    impl Yamlable for &JobSecrets {
        fn as_yaml(&self) -> Yaml {
            match &self.options {
                JobSecretsOptions::Secrets(s) => {
                    let mut hash = Hash::new();
                    for (k, v) in s.iter() {
                        hash.insert_yaml(k, v);
                    }
                    Yaml::Hash(hash)
                }
                JobSecretsOptions::Inherit => Yaml::String("inherit".to_string()),
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct Job {
        name: Option<StringLike>,
        permissions: Option<Permissions>,
        needs: Option<Vec<String>>,
        condition: Option<Either<BooleanExpression, String>>,
        runs_on: Option<RunsOn>,
        snapshot: Option<String>,
        environment: Option<Environment>,
        concurrency: Option<Concurrency>,
        outputs: Option<PyMap<String, StringLike>>,
        env: Option<PyMap<String, StringLike>>,
        defaults: Option<Defaults>,
        steps: Vec<Step>,
        timeout_minutes: Option<i64>,
        strategy: Option<Strategy>,
        continue_on_error: Option<Either<StringLike, BoolLike>>,
        container: Option<Container>,
        services: Option<PyMap<String, Container>>,
        uses: Option<String>,
        with: Option<Hash>,
        secrets: Option<JobSecrets>,
    }
    #[pymethods]
    impl Job {
        #[new]
        #[pyo3(signature = (steps, *, name=None, permissions=None, needs=None, condition=None, runs_on=None, snapshot=None, environment=None, concurrency=None, outputs=None, env=None, defaults=None, timeout_minutes=None, strategy=None, continue_on_error=None, container=None, services=None, uses=None, with_opts=None, secrets=None))]
        fn new(
            steps: Vec<Step>,
            name: Option<StringLike>,
            permissions: Option<Permissions>,
            needs: Option<Vec<String>>,
            condition: Option<Either<BooleanExpression, String>>,
            runs_on: Option<RunsOn>,
            snapshot: Option<String>,
            environment: Option<Environment>,
            concurrency: Option<Concurrency>,
            outputs: Option<PyMap<String, StringLike>>,
            env: Option<PyMap<String, StringLike>>,
            defaults: Option<Defaults>,
            timeout_minutes: Option<i64>,
            strategy: Option<Strategy>,
            continue_on_error: Option<Either<StringLike, BoolLike>>,
            container: Option<Container>,
            services: Option<PyMap<String, Container>>,
            uses: Option<String>,
            with_opts: Option<Bound<PyDict>>,
            secrets: Option<JobSecrets>,
        ) -> PyResult<Self> {
            Ok(Self {
                name,
                permissions,
                needs,
                condition,
                runs_on,
                snapshot,
                environment,
                concurrency,
                outputs,
                env,
                defaults,
                steps,
                timeout_minutes,
                strategy,
                continue_on_error,
                container,
                services,
                uses,
                with: with_opts.map(|w| w.try_as_hash()).transpose()?,
                secrets,
            })
        }
        fn __str__(&self) -> PyResult<String> {
            self.as_yaml_string()
        }
    }
    impl Yamlable for &Job {
        fn as_yaml(&self) -> Yaml {
            let mut out = Hash::new();
            out.insert_yaml_opt("name", &self.name);
            out.insert_yaml_opt("permissions", self.permissions.as_ref());
            out.insert_yaml_opt("needs", &self.needs);
            out.insert_yaml_opt("if", &self.condition);
            out.insert_yaml_opt("runs-on", &self.runs_on);
            out.insert_yaml_opt("snapshot", &self.snapshot);
            out.insert_yaml_opt("environment", &self.environment);
            out.insert_yaml_opt("concurrency", &self.concurrency);
            out.insert_yaml_opt("outputs", &self.outputs);
            out.insert_yaml_opt("env", &self.env);
            if let Some(defaults) = &self.defaults {
                out.insert_yaml_opt("defaults", defaults.maybe_as_yaml());
            }
            out.insert_yaml_opt("strategy", &self.strategy);
            out.insert_yaml("steps", &self.steps);
            out.insert_yaml_opt("timeout-minutes", self.timeout_minutes);
            out.insert_yaml_opt("continue-on-error", &self.continue_on_error);
            out.insert_yaml_opt("container", &self.container);
            out.insert_yaml_opt("services", &self.services);
            out.insert_yaml_opt("uses", &self.uses);
            out.insert_yaml_opt("with", self.with.clone().map(Yaml::Hash));
            out.insert_yaml_opt("secrets", &self.secrets);
            Yaml::Hash(out)
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct BranchProtectionRuleEvent {
        created: bool,
        edited: bool,
        deleted: bool,
    }
    #[pymethods]
    impl BranchProtectionRuleEvent {
        #[new]
        #[pyo3(signature = (*, created=false, edited=false, deleted=false))]
        fn new(created: bool, edited: bool, deleted: bool) -> Self {
            Self {
                created,
                edited,
                deleted,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &BranchProtectionRuleEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            if self.created || self.edited || self.deleted {
                let mut out = Hash::new();
                let mut arr = Array::new();
                arr.push_yaml_cond("created", self.created);
                arr.push_yaml_cond("edited", self.edited);
                arr.push_yaml_cond("deleted", self.deleted);
                out.insert_yaml("types", Yaml::Array(arr));
                Some(Yaml::Hash(out))
            } else {
                None
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct CheckRunEvent {
        created: bool,
        rerequested: bool,
        completed: bool,
        requested_action: bool,
    }
    #[pymethods]
    impl CheckRunEvent {
        #[new]
        #[pyo3(signature = (*, created=false, rerequested=false, completed=false, requested_action=false))]
        fn new(created: bool, rerequested: bool, completed: bool, requested_action: bool) -> Self {
            Self {
                created,
                rerequested,
                completed,
                requested_action,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &CheckRunEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            if self.created || self.rerequested || self.completed || self.requested_action {
                let mut out = Hash::new();
                let mut arr = Array::new();
                arr.push_yaml_cond("created", self.created);
                arr.push_yaml_cond("rerequested", self.rerequested);
                arr.push_yaml_cond("completed", self.completed);
                arr.push_yaml_cond("requested_action", self.requested_action);
                out.insert_yaml("types", Yaml::Array(arr));
                Some(Yaml::Hash(out))
            } else {
                None
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct CheckSuiteEvent {
        created: bool,
    }
    #[pymethods]
    impl CheckSuiteEvent {
        #[new]
        #[pyo3(signature = (*, created=false))]
        fn new(created: bool) -> Self {
            Self { created }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &CheckSuiteEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            if self.created {
                let mut out = Hash::new();
                let mut arr = Array::new();
                arr.push_yaml_cond("created", self.created);
                out.insert_yaml("types", Yaml::Array(arr));
                Some(Yaml::Hash(out))
            } else {
                None
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct DiscussionEvent {
        created: bool,
        edited: bool,
        deleted: bool,
        transferred: bool,
        pinned: bool,
        unpinned: bool,
        labeled: bool,
        unlabeled: bool,
        locked: bool,
        unlocked: bool,
        category_changed: bool,
        answered: bool,
        unanswered: bool,
    }
    #[pymethods]
    impl DiscussionEvent {
        #[new]
        #[pyo3(signature = (*, created=false, edited=false, deleted=false, transferred=false, pinned=false, unpinned=false, labeled=false, unlabeled=false, locked=false, unlocked=false, category_changed=false, answered=false, unanswered=false))]
        fn new(
            created: bool,
            edited: bool,
            deleted: bool,
            transferred: bool,
            pinned: bool,
            unpinned: bool,
            labeled: bool,
            unlabeled: bool,
            locked: bool,
            unlocked: bool,
            category_changed: bool,
            answered: bool,
            unanswered: bool,
        ) -> Self {
            Self {
                created,
                edited,
                deleted,
                transferred,
                pinned,
                unpinned,
                labeled,
                unlabeled,
                locked,
                unlocked,
                category_changed,
                answered,
                unanswered,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &DiscussionEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            if self.created
                || self.edited
                || self.deleted
                || self.transferred
                || self.pinned
                || self.unpinned
                || self.labeled
                || self.unlabeled
                || self.locked
                || self.unlocked
                || self.category_changed
                || self.answered
                || self.unanswered
            {
                let mut out = Hash::new();
                let mut arr = Array::new();
                arr.push_yaml_cond("created", self.created);
                arr.push_yaml_cond("edited", self.edited);
                arr.push_yaml_cond("deleted", self.deleted);
                arr.push_yaml_cond("transferred", self.transferred);
                arr.push_yaml_cond("pinned", self.pinned);
                arr.push_yaml_cond("unpinned", self.unpinned);
                arr.push_yaml_cond("labeled", self.labeled);
                arr.push_yaml_cond("unlabeled", self.unlabeled);
                arr.push_yaml_cond("locked", self.locked);
                arr.push_yaml_cond("unlocked", self.unlocked);
                arr.push_yaml_cond("category_changed", self.category_changed);
                arr.push_yaml_cond("answered", self.answered);
                arr.push_yaml_cond("unanswered", self.unanswered);
                out.insert_yaml("types", Yaml::Array(arr));
                Some(Yaml::Hash(out))
            } else {
                None
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct DiscussionCommentEvent {
        created: bool,
        edited: bool,
        deleted: bool,
    }
    #[pymethods]
    impl DiscussionCommentEvent {
        #[new]
        #[pyo3(signature = (*, created=false, edited=false, deleted=false))]
        fn new(created: bool, edited: bool, deleted: bool) -> Self {
            Self {
                created,
                edited,
                deleted,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &DiscussionCommentEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            if self.created || self.edited || self.deleted {
                let mut out = Hash::new();
                let mut arr = Array::new();
                arr.push_yaml_cond("created", self.created);
                arr.push_yaml_cond("edited", self.edited);
                arr.push_yaml_cond("deleted", self.deleted);
                out.insert_yaml("types", Yaml::Array(arr));
                Some(Yaml::Hash(out))
            } else {
                None
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct ImageVersionEvent {
        names: Option<Vec<String>>,
        versions: Option<Vec<String>>,
    }
    #[pymethods]
    impl ImageVersionEvent {
        #[new]
        #[pyo3(signature = (*, names=None, versions=None))]
        fn new(names: Option<Vec<String>>, versions: Option<Vec<String>>) -> Self {
            let names = names.filter(|v| !v.is_empty());
            let versions = versions.filter(|v| !v.is_empty());
            Self { names, versions }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &ImageVersionEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            if self.names.is_some() || self.versions.is_some() {
                let mut out = Hash::new();
                out.insert_yaml_opt("names", self.names.as_ref());
                out.insert_yaml_opt("versions", self.versions.as_ref());
                Some(Yaml::Hash(out))
            } else {
                None
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct IssueCommentEvent {
        created: bool,
        edited: bool,
        deleted: bool,
    }
    #[pymethods]
    impl IssueCommentEvent {
        #[new]
        #[pyo3(signature = (*, created=false, edited=false, deleted=false))]
        fn new(created: bool, edited: bool, deleted: bool) -> Self {
            Self {
                created,
                edited,
                deleted,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &IssueCommentEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            if self.created || self.edited || self.deleted {
                let mut out = Hash::new();
                let mut arr = Array::new();
                arr.push_yaml_cond("created", self.created);
                arr.push_yaml_cond("edited", self.edited);
                arr.push_yaml_cond("deleted", self.deleted);
                out.insert_yaml("types", Yaml::Array(arr));
                Some(Yaml::Hash(out))
            } else {
                None
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct IssuesEvent {
        created: bool,
        edited: bool,
        deleted: bool,
        transferred: bool,
        pinned: bool,
        unpinned: bool,
        closed: bool,
        reopened: bool,
        assigned: bool,
        unassigned: bool,
        labeled: bool,
        unlabeled: bool,
        locked: bool,
        unlocked: bool,
        milestoned: bool,
        demilestoned: bool,
        typed: bool,
        untyped: bool,
    }
    #[pymethods]
    impl IssuesEvent {
        #[new]
        #[pyo3(signature = (*, created=false, edited=false, deleted=false,  transferred=false, pinned=false, unpinned=false, closed=false, reopened=false, assigned=false, unassigned=false, labeled=false, unlabeled=false, locked=false, unlocked=false, milestoned=false, demilestoned=false, typed=false, untyped=false))]
        fn new(
            created: bool,
            edited: bool,
            deleted: bool,
            transferred: bool,
            pinned: bool,
            unpinned: bool,
            closed: bool,
            reopened: bool,
            assigned: bool,
            unassigned: bool,
            labeled: bool,
            unlabeled: bool,
            locked: bool,
            unlocked: bool,
            milestoned: bool,
            demilestoned: bool,
            typed: bool,
            untyped: bool,
        ) -> Self {
            Self {
                created,
                edited,
                deleted,
                transferred,
                pinned,
                unpinned,
                closed,
                reopened,
                assigned,
                unassigned,
                labeled,
                unlabeled,
                locked,
                unlocked,
                milestoned,
                demilestoned,
                typed,
                untyped,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &IssuesEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            if self.created
                || self.edited
                || self.deleted
                || self.transferred
                || self.pinned
                || self.unpinned
                || self.closed
                || self.reopened
                || self.assigned
                || self.unassigned
                || self.labeled
                || self.unlabeled
                || self.locked
                || self.unlocked
                || self.milestoned
                || self.demilestoned
                || self.typed
                || self.untyped
            {
                let mut out = Hash::new();
                let mut arr = Array::new();
                arr.push_yaml_cond("created", self.created);
                arr.push_yaml_cond("edited", self.edited);
                arr.push_yaml_cond("deleted", self.deleted);
                arr.push_yaml_cond("transferred", self.transferred);
                arr.push_yaml_cond("pinned", self.pinned);
                arr.push_yaml_cond("unpinned", self.unpinned);
                arr.push_yaml_cond("closed", self.closed);
                arr.push_yaml_cond("reopened", self.reopened);
                arr.push_yaml_cond("assigned", self.assigned);
                arr.push_yaml_cond("unassigned", self.unassigned);
                arr.push_yaml_cond("labeled", self.labeled);
                arr.push_yaml_cond("unlabeled", self.unlabeled);
                arr.push_yaml_cond("locked", self.locked);
                arr.push_yaml_cond("unlocked", self.unlocked);
                arr.push_yaml_cond("milestoned", self.milestoned);
                arr.push_yaml_cond("demilestoned", self.demilestoned);
                arr.push_yaml_cond("typed", self.typed);
                arr.push_yaml_cond("untyped", self.untyped);
                out.insert_yaml("types", Yaml::Array(arr));
                Some(Yaml::Hash(out))
            } else {
                None
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct LabelEvent {
        created: bool,
        edited: bool,
        deleted: bool,
    }
    #[pymethods]
    impl LabelEvent {
        #[new]
        #[pyo3(signature = (*, created=false, edited=false, deleted=false))]
        fn new(created: bool, edited: bool, deleted: bool) -> Self {
            Self {
                created,
                edited,
                deleted,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &LabelEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            if self.created || self.edited || self.deleted {
                let mut out = Hash::new();
                let mut arr = Array::new();
                arr.push_yaml_cond("created", self.created);
                arr.push_yaml_cond("edited", self.edited);
                arr.push_yaml_cond("deleted", self.deleted);
                out.insert_yaml("types", Yaml::Array(arr));
                Some(Yaml::Hash(out))
            } else {
                None
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct MergeGroupEvent {
        checks_requested: bool,
    }
    #[pymethods]
    impl MergeGroupEvent {
        #[new]
        #[pyo3(signature = (*, checks_requested=false))]
        fn new(checks_requested: bool) -> Self {
            Self { checks_requested }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &MergeGroupEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            if self.checks_requested {
                let mut out = Hash::new();
                let mut arr = Array::new();
                arr.push_yaml_cond("checks_requested", self.checks_requested);
                out.insert_yaml("types", Yaml::Array(arr));
                Some(Yaml::Hash(out))
            } else {
                None
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct MilestoneEvent {
        created: bool,
        closed: bool,
        opened: bool,
        edited: bool,
        deleted: bool,
    }
    #[pymethods]
    impl MilestoneEvent {
        #[new]
        #[pyo3(signature = (*, created=false, closed=false, opened=false, edited=false, deleted=false))]
        fn new(created: bool, closed: bool, opened: bool, edited: bool, deleted: bool) -> Self {
            Self {
                created,
                closed,
                opened,
                edited,
                deleted,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &MilestoneEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            if self.created || self.closed || self.opened || self.edited || self.deleted {
                let mut out = Hash::new();
                let mut arr = Array::new();
                arr.push_yaml_cond("created", self.created);
                arr.push_yaml_cond("closed", self.closed);
                arr.push_yaml_cond("opened", self.opened);
                arr.push_yaml_cond("edited", self.edited);
                arr.push_yaml_cond("deleted", self.deleted);
                out.insert_yaml("types", Yaml::Array(arr));
                Some(Yaml::Hash(out))
            } else {
                None
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct PullRequestEvent {
        assigned: bool,
        unassigned: bool,
        labeled: bool,
        unlabeled: bool,
        opened: bool,
        edited: bool,
        closed: bool,
        reopened: bool,
        synchronize: bool,
        converted_to_draft: bool,
        locked: bool,
        unlocked: bool,
        enqueued: bool,
        dequeued: bool,
        milestoned: bool,
        demilestoned: bool,
        ready_for_review: bool,
        review_requested: bool,
        review_request_removed: bool,
        auto_merge_enabled: bool,
        auto_merge_disabled: bool,
        branches: Option<Vec<String>>,
        branches_ignore: Option<Vec<String>>,
        paths: Option<Vec<String>>,
        paths_ignore: Option<Vec<String>>,
    }
    #[pymethods]
    impl PullRequestEvent {
        #[new]
        #[pyo3(signature = (*, branches=None, branches_ignore=None, paths=None, paths_ignore=None, assigned=false, unassigned=false, labeled=false, unlabeled=false, opened=false, edited=false, closed=false, reopened=false, synchronize=false, converted_to_draft=false, locked=false, unlocked=false, enqueued=false, dequeued=false, milestoned=false, demilestoned=false, ready_for_review=false, review_requested=false, review_request_removed=false, auto_merge_enabled=false, auto_merge_disabled=false))]
        fn new(
            branches: Option<Vec<String>>,
            branches_ignore: Option<Vec<String>>,
            paths: Option<Vec<String>>,
            paths_ignore: Option<Vec<String>>,
            assigned: bool,
            unassigned: bool,
            labeled: bool,
            unlabeled: bool,
            opened: bool,
            edited: bool,
            closed: bool,
            reopened: bool,
            synchronize: bool,
            converted_to_draft: bool,
            locked: bool,
            unlocked: bool,
            enqueued: bool,
            dequeued: bool,
            milestoned: bool,
            demilestoned: bool,
            ready_for_review: bool,
            review_requested: bool,
            review_request_removed: bool,
            auto_merge_enabled: bool,
            auto_merge_disabled: bool,
        ) -> Self {
            let branches = branches.filter(|v| !v.is_empty());
            let branches_ignore = branches_ignore.filter(|v| !v.is_empty());
            let paths = paths.filter(|v| !v.is_empty());
            let paths_ignore = paths_ignore.filter(|v| !v.is_empty());
            Self {
                branches,
                branches_ignore,
                paths,
                paths_ignore,
                assigned,
                unassigned,
                labeled,
                unlabeled,
                opened,
                edited,
                closed,
                reopened,
                synchronize,
                converted_to_draft,
                locked,
                unlocked,
                enqueued,
                dequeued,
                milestoned,
                demilestoned,
                ready_for_review,
                review_requested,
                review_request_removed,
                auto_merge_enabled,
                auto_merge_disabled,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &PullRequestEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            let mut out = Hash::new();
            out.insert_yaml_opt("branches", self.branches.as_ref());
            out.insert_yaml_opt("branches-ignore", self.branches_ignore.as_ref());
            out.insert_yaml_opt("paths", self.paths.as_ref());
            out.insert_yaml_opt("paths-ignore", self.paths_ignore.as_ref());
            if self.assigned
                || self.unassigned
                || self.labeled
                || self.unlabeled
                || self.opened
                || self.edited
                || self.closed
                || self.reopened
                || self.synchronize
                || self.converted_to_draft
                || self.locked
                || self.unlocked
                || self.enqueued
                || self.dequeued
                || self.milestoned
                || self.demilestoned
                || self.ready_for_review
                || self.review_requested
                || self.review_request_removed
                || self.auto_merge_enabled
                || self.auto_merge_disabled
            {
                let mut arr = Array::new();
                arr.push_yaml_cond("assigned", self.assigned);
                arr.push_yaml_cond("unassigned", self.unassigned);
                arr.push_yaml_cond("labeled", self.labeled);
                arr.push_yaml_cond("unlabeled", self.unlabeled);
                arr.push_yaml_cond("opened", self.opened);
                arr.push_yaml_cond("edited", self.edited);
                arr.push_yaml_cond("closed", self.closed);
                arr.push_yaml_cond("reopened", self.reopened);
                arr.push_yaml_cond("synchronize", self.synchronize);
                arr.push_yaml_cond("converted_to_draft", self.converted_to_draft);
                arr.push_yaml_cond("locked", self.locked);
                arr.push_yaml_cond("unlocked", self.unlocked);
                arr.push_yaml_cond("enqueued", self.enqueued);
                arr.push_yaml_cond("dequeued", self.dequeued);
                arr.push_yaml_cond("milestoned", self.milestoned);
                arr.push_yaml_cond("demilestoned", self.demilestoned);
                arr.push_yaml_cond("ready_for_review", self.ready_for_review);
                arr.push_yaml_cond("review_requested", self.review_requested);
                arr.push_yaml_cond("review_request_removed", self.review_request_removed);
                arr.push_yaml_cond("auto_merge_enabled", self.auto_merge_enabled);
                arr.push_yaml_cond("auto_merge_disabled", self.auto_merge_disabled);
                out.insert_yaml("types", Yaml::Array(arr));
            }
            if out.is_empty() {
                None
            } else {
                Some(Yaml::Hash(out))
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct PullRequestReviewEvent {
        submitted: bool,
        edited: bool,
        dismissed: bool,
    }
    #[pymethods]
    impl PullRequestReviewEvent {
        #[new]
        #[pyo3(signature = (*, submitted=false, edited=false, dismissed=false))]
        fn new(submitted: bool, edited: bool, dismissed: bool) -> Self {
            Self {
                submitted,
                edited,
                dismissed,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &PullRequestReviewEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            if self.submitted || self.edited || self.dismissed {
                let mut out = Hash::new();
                let mut arr = Array::new();
                arr.push_yaml_cond("submitted", self.submitted);
                arr.push_yaml_cond("edited", self.edited);
                arr.push_yaml_cond("dismissed", self.dismissed);
                out.insert_yaml("types", Yaml::Array(arr));
                Some(Yaml::Hash(out))
            } else {
                None
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct PullRequestReviewCommentEvent {
        created: bool,
        edited: bool,
        deleted: bool,
    }
    #[pymethods]
    impl PullRequestReviewCommentEvent {
        #[new]
        #[pyo3(signature = (*, created=false,edited=false, deleted=false))]
        fn new(created: bool, edited: bool, deleted: bool) -> Self {
            Self {
                created,
                edited,
                deleted,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &PullRequestReviewCommentEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            if self.created || self.edited || self.deleted {
                let mut out = Hash::new();
                let mut arr = Array::new();
                arr.push_yaml_cond("created", self.created);
                arr.push_yaml_cond("edited", self.edited);
                arr.push_yaml_cond("deleted", self.deleted);
                out.insert_yaml("types", Yaml::Array(arr));
                Some(Yaml::Hash(out))
            } else {
                None
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct PushEvent {
        branches: Option<Vec<String>>,
        branches_ignore: Option<Vec<String>>,
        tags: Option<Vec<String>>,
        tags_ignore: Option<Vec<String>>,
        paths: Option<Vec<String>>,
        paths_ignore: Option<Vec<String>>,
    }
    #[pymethods]
    impl PushEvent {
        #[new]
        #[pyo3(signature = (*, branches=None, branches_ignore=None, tags=None, tags_ignore=None, paths=None, paths_ignore=None))]
        fn new(
            branches: Option<Vec<String>>,
            branches_ignore: Option<Vec<String>>,
            tags: Option<Vec<String>>,
            tags_ignore: Option<Vec<String>>,
            paths: Option<Vec<String>>,
            paths_ignore: Option<Vec<String>>,
        ) -> Self {
            Self {
                branches,
                branches_ignore,
                tags,
                tags_ignore,
                paths,
                paths_ignore,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &PushEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            let mut out = Hash::new();
            out.insert_yaml_opt("branches", self.branches.as_ref());
            out.insert_yaml_opt("branches-ignore", self.branches_ignore.as_ref());
            out.insert_yaml_opt("tags", self.tags.as_ref());
            out.insert_yaml_opt("tags-ignore", self.tags_ignore.as_ref());
            out.insert_yaml_opt("paths", self.paths.as_ref());
            out.insert_yaml_opt("paths-ignore", self.paths_ignore.as_ref());
            if out.is_empty() {
                None
            } else {
                Some(Yaml::Hash(out))
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct RegistryPackageEvent {
        published: bool,
        updated: bool,
    }
    #[pymethods]
    impl RegistryPackageEvent {
        #[new]
        #[pyo3(signature = (*, published=false, updated=false))]
        fn new(published: bool, updated: bool) -> Self {
            Self { published, updated }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &RegistryPackageEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            if self.published || self.updated {
                let mut out = Hash::new();
                let mut arr = Array::new();
                arr.push_yaml_cond("published", self.published);
                arr.push_yaml_cond("updated", self.updated);
                out.insert_yaml("types", Yaml::Array(arr));
                Some(Yaml::Hash(out))
            } else {
                None
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct ReleaseEvent {
        published: bool,
        unpublished: bool,
        created: bool,
        edited: bool,
        deleted: bool,
        prereleased: bool,
        released: bool,
    }
    #[pymethods]
    impl ReleaseEvent {
        #[new]
        #[pyo3(signature = (*, published=false, unpublished=false, created=false, edited=false, deleted=false, prereleased=false, released=false))]
        fn new(
            published: bool,
            unpublished: bool,
            created: bool,
            edited: bool,
            deleted: bool,
            prereleased: bool,
            released: bool,
        ) -> Self {
            Self {
                published,
                unpublished,
                created,
                edited,
                deleted,
                prereleased,
                released,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &ReleaseEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            if self.published
                || self.unpublished
                || self.created
                || self.edited
                || self.deleted
                || self.prereleased
                || self.released
            {
                let mut out = Hash::new();
                let mut arr = Array::new();
                arr.push_yaml_cond("published", self.published);
                arr.push_yaml_cond("unpublished", self.unpublished);
                arr.push_yaml_cond("created", self.created);
                arr.push_yaml_cond("edited", self.edited);
                arr.push_yaml_cond("deleted", self.deleted);
                arr.push_yaml_cond("prereleased", self.prereleased);
                arr.push_yaml_cond("released", self.released);
                out.insert_yaml("types", Yaml::Array(arr));
                Some(Yaml::Hash(out))
            } else {
                None
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct RepositoryDispatchEvent {
        types: Option<Vec<String>>,
    }
    #[pymethods]
    impl RepositoryDispatchEvent {
        #[new]
        #[pyo3(signature = (*, types=None))]
        fn new(types: Option<Vec<String>>) -> Self {
            let types = types.filter(|v| !v.is_empty());
            Self { types }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &RepositoryDispatchEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            let mut out = Hash::new();
            out.insert_yaml_opt("types", self.types.as_ref());
            if out.is_empty() {
                None
            } else {
                Some(Yaml::Hash(out))
            }
        }
    }

    #[derive(Clone)]
    enum CronStepType {
        Value(u8),
        List(Vec<u8>),
        Range(u8, u8),
        Step { start: Option<u8>, step: u8 },
    }
    impl Display for CronStepType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Self::Value(v) => v.to_string(),
                    Self::List(items) => items
                        .iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(","),

                    Self::Range(min, max) => format!("{min}-{max}"),
                    Self::Step { start, step } => format!(
                        "{}/{}",
                        start.map(|s| s.to_string()).unwrap_or("*".to_string()),
                        step
                    ),
                }
            )
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    struct CronMinute(u8);

    impl<'a, 'py> FromPyObject<'a, 'py> for CronMinute {
        type Error = PyErr;
        fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
            if let Ok(num) = obj.extract::<u8>()
                && num <= 59 {
                    return Ok(CronMinute(num));
                }
            Err(PyValueError::new_err(
                "Minute must be an integer in range 0..=59",
            ))
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    struct CronHour(u8);

    impl<'a, 'py> FromPyObject<'a, 'py> for CronHour {
        type Error = PyErr;
        fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
            if let Ok(num) = obj.extract::<u8>()
                && num <= 23 {
                    return Ok(CronHour(num));
                }
            Err(PyValueError::new_err(
                "Hour must be an integer in range 0..=23",
            ))
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    struct CronDay(u8);

    impl<'a, 'py> FromPyObject<'a, 'py> for CronDay {
        type Error = PyErr;
        fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
            if let Ok(num) = obj.extract::<u8>()
                && (1..=31).contains(&num) {
                    return Ok(CronDay(num));
                }
            Err(PyValueError::new_err(
                "Hour must be an integer in range 1..=31",
            ))
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    struct CronMonth(u8);

    impl<'a, 'py> FromPyObject<'a, 'py> for CronMonth {
        type Error = PyErr;
        fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
            let msg = "Month must be an integer in range 1..=12";
            if let Ok(num) = obj.extract::<u8>()
                && (1..=12).contains(&num) {
                    return Ok(CronMonth(num));
                }
            Err(PyValueError::new_err(msg))
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    struct CronDayOfWeek(u8);

    impl<'a, 'py> FromPyObject<'a, 'py> for CronDayOfWeek {
        type Error = PyErr;
        fn extract(obj: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
            let msg = "Day of week must be an integer in range 0..=6 (0=Sunday)";
            if let Ok(num) = obj.extract::<u8>()
                && num <= 6 {
                    return Ok(CronDayOfWeek(num));
                }
            Err(PyValueError::new_err(msg))
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct Minute(CronStepType);
    impl Display for Minute {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    #[pymethods]
    impl Minute {
        #[new]
        fn new(minute: Bound<PyAny>) -> PyResult<Self> {
            if let Ok(l) = minute.extract::<Bound<PyList>>() {
                let mut res = Vec::new();
                for item in l.iter() {
                    let item = item.extract::<CronMinute>()?;
                    res.push(item.0);
                }
                return Ok(Self(CronStepType::List(res)));
            }
            let minute = minute.extract::<CronMinute>()?;
            Ok(Self(CronStepType::Value(minute.0)))
        }
        #[staticmethod]
        fn between(start: Bound<PyAny>, end: Bound<PyAny>) -> PyResult<Self> {
            let min = start.extract::<CronMinute>()?;
            let max = end.extract::<CronMinute>()?;
            Ok(Self(CronStepType::Range(min.0, max.0)))
        }
        #[staticmethod]
        #[pyo3(signature = (interval, *, start = None))]
        fn every(interval: Bound<PyAny>, start: Option<Bound<PyAny>>) -> PyResult<Self> {
            let start = start
                .map(|a| a.extract::<CronMinute>())
                .transpose()?
                .map(|s| s.0);
            let interval = interval.extract::<CronMinute>()?;
            Ok(Self(CronStepType::Step {
                start,
                step: interval.0,
            }))
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct Hour(CronStepType);
    impl Display for Hour {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    #[pymethods]
    impl Hour {
        #[new]
        fn new(hour: Bound<PyAny>) -> PyResult<Self> {
            if let Ok(l) = hour.extract::<Bound<PyList>>() {
                let mut res = Vec::new();
                for item in l.iter() {
                    let item = item.extract::<CronHour>()?;
                    res.push(item.0);
                }
                return Ok(Self(CronStepType::List(res)));
            }
            let hour = hour.extract::<CronHour>()?;
            Ok(Self(CronStepType::Value(hour.0)))
        }
        #[staticmethod]
        fn between(start: Bound<PyAny>, end: Bound<PyAny>) -> PyResult<Self> {
            let min = start.extract::<CronHour>()?;
            let max = end.extract::<CronHour>()?;
            Ok(Self(CronStepType::Range(min.0, max.0)))
        }
        #[staticmethod]
        #[pyo3(signature = (interval, *, start = None))]
        fn every(interval: Bound<PyAny>, start: Option<Bound<PyAny>>) -> PyResult<Self> {
            let start = start
                .map(|a| a.extract::<CronHour>())
                .transpose()?
                .map(|s| s.0);
            let interval = interval.extract::<CronHour>()?;
            Ok(Self(CronStepType::Step {
                start,
                step: interval.0,
            }))
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct Day(CronStepType);
    impl Display for Day {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    #[pymethods]
    impl Day {
        #[new]
        fn new(day: Bound<PyAny>) -> PyResult<Self> {
            if let Ok(l) = day.extract::<Bound<PyList>>() {
                let mut res = Vec::new();
                for item in l.iter() {
                    let item = item.extract::<CronDay>()?;
                    res.push(item.0);
                }
                return Ok(Self(CronStepType::List(res)));
            }
            let day = day.extract::<CronDay>()?;
            Ok(Self(CronStepType::Value(day.0)))
        }
        #[staticmethod]
        fn between(min: Bound<PyAny>, max: Bound<PyAny>) -> PyResult<Self> {
            let min = min.extract::<CronDay>()?;
            let max = max.extract::<CronDay>()?;
            Ok(Self(CronStepType::Range(min.0, max.0)))
        }
        #[staticmethod]
        #[pyo3(signature = (interval, *, start = None))]
        fn every(interval: Bound<PyAny>, start: Option<Bound<PyAny>>) -> PyResult<Self> {
            let start = start
                .map(|a| a.extract::<CronDay>())
                .transpose()?
                .map(|s| s.0);
            let interval = interval.extract::<CronDay>()?;
            Ok(Self(CronStepType::Step {
                start,
                step: interval.0,
            }))
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct Month(CronStepType);
    impl Display for Month {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    #[pymethods]
    impl Month {
        #[new]
        fn new(month: Bound<PyAny>) -> PyResult<Self> {
            if let Ok(l) = month.extract::<Bound<PyList>>() {
                let mut res = Vec::new();
                for item in l.iter() {
                    let item = item.extract::<CronMonth>()?;
                    res.push(item.0);
                }
                return Ok(Self(CronStepType::List(res)));
            }
            let month = month.extract::<CronMonth>()?;
            Ok(Self(CronStepType::Value(month.0)))
        }
        #[staticmethod]
        fn between(min: Bound<PyAny>, max: Bound<PyAny>) -> PyResult<Self> {
            let min = min.extract::<CronMonth>()?;
            let max = max.extract::<CronMonth>()?;
            Ok(Self(CronStepType::Range(min.0, max.0)))
        }
        #[staticmethod]
        #[pyo3(signature = (interval, *, start = None))]
        fn every(interval: Bound<PyAny>, start: Option<Bound<PyAny>>) -> PyResult<Self> {
            let start = start
                .map(|a| a.extract::<CronMonth>())
                .transpose()?
                .map(|s| s.0);
            let interval = interval.extract::<CronMonth>()?;
            Ok(Self(CronStepType::Step {
                start,
                step: interval.0,
            }))
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct DayOfWeek(CronStepType);
    impl Display for DayOfWeek {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    #[pymethods]
    impl DayOfWeek {
        #[new]
        fn new(day_of_week: Bound<PyAny>) -> PyResult<Self> {
            if let Ok(l) = day_of_week.extract::<Bound<PyList>>() {
                let mut res = Vec::new();
                for item in l.iter() {
                    let item = item.extract::<CronDayOfWeek>()?;
                    res.push(item.0);
                }
                return Ok(Self(CronStepType::List(res)));
            }
            let day_of_week = day_of_week.extract::<CronMonth>()?;
            Ok(Self(CronStepType::Value(day_of_week.0)))
        }
        #[staticmethod]
        fn between(min: Bound<PyAny>, max: Bound<PyAny>) -> PyResult<Self> {
            let min = min.extract::<CronDayOfWeek>()?;
            let max = max.extract::<CronDayOfWeek>()?;
            Ok(Self(CronStepType::Range(min.0, max.0)))
        }
        #[staticmethod]
        #[pyo3(signature = (interval, *, start = None))]
        fn every(interval: Bound<PyAny>, start: Option<Bound<PyAny>>) -> PyResult<Self> {
            let start = start
                .map(|a| a.extract::<CronDayOfWeek>())
                .transpose()?
                .map(|s| s.0);
            let interval = interval.extract::<CronDayOfWeek>()?;
            Ok(Self(CronStepType::Step {
                start,
                step: interval.0,
            }))
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct Cron {
        minute: Option<Minute>,
        hour: Option<Hour>,
        day: Option<Day>,
        month: Option<Month>,
        day_of_week: Option<DayOfWeek>,
    }
    #[pymethods]
    impl Cron {
        #[new]
        #[pyo3(signature = (*, minute = None, hour = None, day = None, month = None, day_of_week = None))]
        fn new(
            minute: Option<Minute>,
            hour: Option<Hour>,
            day: Option<Day>,
            month: Option<Month>,
            day_of_week: Option<DayOfWeek>,
        ) -> Self {
            Self {
                minute,
                hour,
                day,
                month,
                day_of_week,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.as_yaml_string()
        }
    }
    impl Yamlable for &Cron {
        fn as_yaml(&self) -> Yaml {
            let mut out = Hash::new();
            let s = format!(
                "{} {} {} {} {}",
                self.minute
                    .clone()
                    .map(|s| s.to_string())
                    .unwrap_or("*".to_string()),
                self.hour
                    .clone()
                    .map(|s| s.to_string())
                    .unwrap_or("*".to_string()),
                self.day
                    .clone()
                    .map(|s| s.to_string())
                    .unwrap_or("*".to_string()),
                self.month
                    .clone()
                    .map(|s| s.to_string())
                    .unwrap_or("*".to_string()),
                self.day_of_week
                    .clone()
                    .map(|s| s.to_string())
                    .unwrap_or("*".to_string())
            );
            out.insert_yaml("cron", s);
            Yaml::Hash(out)
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct ScheduleEvent {
        crons: Option<Vec<Cron>>,
    }
    #[pymethods]
    impl ScheduleEvent {
        #[new]
        #[pyo3(signature = (*, crons=None))]
        fn new(crons: Option<Vec<Cron>>) -> Self {
            let crons = crons.filter(|v| !v.is_empty());
            Self { crons }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &ScheduleEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            let mut out = Array::new();
            if let Some(crons) = &self.crons {
                for cron in crons {
                    out.push_yaml(cron);
                }
                Some(Yaml::Array(out))
            } else {
                None
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct WatchEvent {
        started: bool,
    }
    #[pymethods]
    impl WatchEvent {
        #[new]
        #[pyo3(signature = (*, started=false))]
        fn new(started: bool) -> Self {
            Self { started }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &WatchEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            if self.started {
                let mut out = Hash::new();
                let mut arr = Array::new();
                arr.push_yaml_cond("started", self.started);
                out.insert_yaml("types", Yaml::Array(arr));
                Some(Yaml::Hash(out))
            } else {
                None
            }
        }
    }

    #[derive(Clone)]
    enum WorkflowInputType {
        Boolean { default: Option<BoolLike> },
        Number { default: Option<IntLike> },
        String { default: Option<StringLike> },
    }
    impl WorkflowInputType {
        fn get_type(&self) -> Yaml {
            match self {
                Self::Boolean { .. } => Yaml::from_str("boolean"),
                Self::Number { .. } => Yaml::from_str("number"),
                Self::String { .. } => Yaml::from_str("string"),
            }
        }
        fn get_default(&self) -> Option<Yaml> {
            match self {
                Self::Boolean { default } => default.clone().map(|b| b.as_yaml()),
                Self::Number { default } => default.clone().map(|n| n.as_yaml()),
                Self::String { default } => default.clone().map(|s| s.as_yaml()),
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct WorkflowInput {
        description: Option<String>,
        input_type: WorkflowInputType,
        required: Option<bool>,
    }
    #[pymethods]
    impl WorkflowInput {
        #[staticmethod]
        #[pyo3(signature = (*, description=None, default=None, required=None))]
        fn boolean(
            description: Option<String>,
            default: Option<BoolLike>,
            required: Option<bool>,
        ) -> Self {
            Self {
                description,
                input_type: WorkflowInputType::Boolean { default },
                required,
            }
        }
        #[staticmethod]
        #[pyo3(signature = (*, description=None, default=None, required=None))]
        fn number(
            description: Option<String>,
            default: Option<IntLike>,
            required: Option<bool>,
        ) -> Self {
            Self {
                description,
                input_type: WorkflowInputType::Number { default },
                required,
            }
        }
        #[staticmethod]
        #[pyo3(signature = (*, description=None, default=None, required=None))]
        fn string(
            description: Option<String>,
            default: Option<StringLike>,
            required: Option<bool>,
        ) -> Self {
            Self {
                description,
                input_type: WorkflowInputType::String { default },
                required,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.as_yaml_string()
        }
    }
    impl Yamlable for &WorkflowInput {
        fn as_yaml(&self) -> Yaml {
            let mut out = Hash::new();
            out.insert_yaml_opt("description", &self.description);
            out.insert_yaml("type", self.input_type.get_type());
            out.insert_yaml_opt("required", self.required);
            out.insert_yaml_opt("default", self.input_type.get_default());
            Yaml::Hash(out)
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct WorkflowOutput {
        description: Option<String>,
        value: StringLike,
    }
    #[pymethods]
    impl WorkflowOutput {
        #[new]
        #[pyo3(signature = (value, *, description=None))]
        fn new(value: StringLike, description: Option<String>) -> Self {
            Self { value, description }
        }

        fn __str__(&self) -> PyResult<String> {
            self.as_yaml_string()
        }
    }
    impl Yamlable for &WorkflowOutput {
        fn as_yaml(&self) -> Yaml {
            let mut out = Hash::new();
            out.insert_yaml_opt("description", &self.description);
            out.insert_yaml("value", &self.value);
            Yaml::Hash(out)
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct WorkflowSecret {
        description: Option<String>,
        required: Option<bool>,
    }
    #[pymethods]
    impl WorkflowSecret {
        #[new]
        #[pyo3(signature = (*, description=None, required=None))]
        fn new(description: Option<String>, required: Option<bool>) -> Self {
            Self {
                description,
                required,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &WorkflowSecret {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            let mut out = Hash::new();
            out.insert_yaml_opt("description", &self.description);
            out.insert_yaml_opt("required", self.required);
            if out.is_empty() {
                None
            } else {
                Some(Yaml::Hash(out))
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct WorkflowCallEvent {
        inputs: PyMap<String, WorkflowInput>,
        outputs: PyMap<String, WorkflowOutput>,
        secrets: PyMap<String, WorkflowSecret>,
    }
    #[pymethods]
    impl WorkflowCallEvent {
        #[new]
        #[pyo3(signature = (*, inputs=None, outputs=None, secrets=None))]
        fn new(
            inputs: Option<PyMap<String, WorkflowInput>>,
            outputs: Option<PyMap<String, WorkflowOutput>>,
            secrets: Option<PyMap<String, WorkflowSecret>>,
        ) -> Self {
            Self {
                inputs: inputs.unwrap_or_default(),
                outputs: outputs.unwrap_or_default(),
                secrets: secrets.unwrap_or_default(),
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for WorkflowCallEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            let mut out = Hash::new();
            let mut inputs = Hash::new();
            for (k, v) in self.inputs.iter() {
                inputs.insert_yaml(k, v)
            }
            if !inputs.is_empty() {
                out.insert_yaml("inputs", Yaml::Hash(inputs));
            }
            let mut outputs = Hash::new();
            for (k, v) in self.outputs.iter() {
                outputs.insert_yaml(k, v)
            }
            if !outputs.is_empty() {
                out.insert_yaml("outputs", Yaml::Hash(outputs));
            }
            let mut secrets = Hash::new();
            for (k, v) in self.secrets.iter() {
                secrets.insert_yaml(k, v.maybe_as_yaml().unwrap_or(Yaml::Null))
            }
            if !secrets.is_empty() {
                out.insert_yaml("secrets", Yaml::Hash(secrets));
            }
            if out.is_empty() {
                None
            } else {
                Some(Yaml::Hash(out))
            }
        }
    }

    #[derive(Clone)]
    enum WorkflowDispatchInputType {
        Boolean {
            default: Option<bool>,
        },
        Choice {
            default: Option<String>,
            options: Vec<String>,
        },
        Number {
            default: Option<i64>,
        },
        Environment,
        String {
            default: Option<String>,
        },
    }
    impl WorkflowDispatchInputType {
        fn get_type(&self) -> Yaml {
            match self {
                Self::Boolean { .. } => Yaml::from_str("boolean"),
                Self::Choice { .. } => Yaml::from_str("choice"),
                Self::Number { .. } => Yaml::from_str("number"),
                Self::Environment => Yaml::from_str("environment"),
                Self::String { .. } => Yaml::from_str("string"),
            }
        }
        fn get_default(&self) -> Option<Yaml> {
            match self {
                Self::Boolean { default } => default.map(Yaml::Boolean),
                Self::Choice { default, .. } => default.clone().map(Yaml::String),
                Self::Number { default } => default.map(Yaml::Integer),
                Self::Environment => None, // TODO: check if environment can have a default
                Self::String { default } => default.clone().map(Yaml::String),
            }
        }
        fn get_options(&self) -> Option<Yaml> {
            if let Self::Choice { options, .. } = self {
                Some(Yaml::Array(
                    options.iter().map(|s| Yaml::String(s.clone())).collect(),
                ))
            } else {
                None
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct WorkflowDispatchInput {
        description: Option<String>,
        input_type: WorkflowDispatchInputType,
        required: Option<bool>,
    }
    #[pymethods]
    impl WorkflowDispatchInput {
        #[staticmethod]
        #[pyo3(signature = (*, description=None, default=None, required=None))]
        fn boolean(
            description: Option<String>,
            default: Option<bool>,
            required: Option<bool>,
        ) -> Self {
            Self {
                description,
                input_type: WorkflowDispatchInputType::Boolean { default },
                required,
            }
        }
        #[staticmethod]
        #[pyo3(signature = (options, *, description=None, default=None, required=None))]
        fn choice(
            options: Vec<String>,
            description: Option<String>,
            default: Option<String>,
            required: Option<bool>,
        ) -> Self {
            Self {
                description,
                input_type: WorkflowDispatchInputType::Choice { default, options },
                required,
            }
        }
        #[staticmethod]
        #[pyo3(signature = (*, description=None, default=None, required=None))]
        fn number(
            description: Option<String>,
            default: Option<i64>,
            required: Option<bool>,
        ) -> Self {
            Self {
                description,
                input_type: WorkflowDispatchInputType::Number { default },
                required,
            }
        }
        #[staticmethod]
        #[pyo3(signature = (*, description=None, required=None))]
        fn environment(description: Option<String>, required: Option<bool>) -> Self {
            Self {
                description,
                input_type: WorkflowDispatchInputType::Environment,
                required,
            }
        }
        #[staticmethod]
        #[pyo3(signature = (*, description=None, default=None, required=None))]
        fn string(
            description: Option<String>,
            default: Option<String>,
            required: Option<bool>,
        ) -> Self {
            Self {
                description,
                input_type: WorkflowDispatchInputType::String { default },
                required,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.as_yaml_string()
        }
    }
    impl Yamlable for &WorkflowDispatchInput {
        fn as_yaml(&self) -> Yaml {
            let mut out = Hash::new();
            out.insert_yaml_opt("description", &self.description);
            out.insert_yaml("type", self.input_type.get_type());
            out.insert_yaml_opt("required", self.required);
            out.insert_yaml_opt("default", self.input_type.get_default());
            out.insert_yaml_opt("options", self.input_type.get_options());
            Yaml::Hash(out)
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct WorkflowDispatchEvent {
        inputs: Option<PyMap<String, WorkflowDispatchInput>>,
    }
    #[pymethods]
    impl WorkflowDispatchEvent {
        #[new]
        #[pyo3(signature = (*, inputs=None))]
        fn new(inputs: Option<PyMap<String, WorkflowDispatchInput>>) -> Self {
            Self {
                inputs: inputs.filter(|i| !i.is_empty()),
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &WorkflowDispatchEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            if let Some(inputs) = &self.inputs {
                let mut out = Hash::new();
                for (k, v) in inputs.iter() {
                    out.insert_yaml(k, v);
                }
                Some(Yaml::Hash(out))
            } else {
                None
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct WorkflowRunEvent {
        workflows: Option<Vec<String>>,
        completed: bool,
        requested: bool,
        in_progress: bool,
        branches: Option<Vec<String>>,
        branches_ignore: Option<Vec<String>>,
    }
    #[pymethods]
    impl WorkflowRunEvent {
        #[new]
        #[pyo3(signature = (*, workflows=None, completed=false, requested=false, in_progress=false, branches=None, branches_ignore=None))]
        fn new(
            workflows: Option<Vec<String>>,
            completed: bool,
            requested: bool,
            in_progress: bool,
            branches: Option<Vec<String>>,
            branches_ignore: Option<Vec<String>>,
        ) -> Self {
            let workflows = workflows.filter(|w| !w.is_empty());
            let branches = branches.filter(|b| !b.is_empty());
            let branches_ignore = branches_ignore.filter(|b| !b.is_empty());
            Self {
                workflows,
                completed,
                requested,
                in_progress,
                branches,
                branches_ignore,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &WorkflowRunEvent {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            let mut out = Hash::new();
            out.insert_yaml_opt("workflows", &self.workflows);
            if self.completed || self.requested || self.in_progress {
                let mut types = Array::new();
                types.push_yaml_cond("completed", self.completed);
                types.push_yaml_cond("requested", self.requested);
                types.push_yaml_cond("in_progress", self.in_progress);
                out.insert_yaml("types", Yaml::Array(types));
            }
            out.insert_yaml_opt("branches", &self.branches);
            out.insert_yaml_opt("branches-ignore", &self.branches_ignore);
            if out.is_empty() {
                None
            } else {
                Some(Yaml::Hash(out))
            }
        }
    }

    #[pyclass]
    #[derive(Clone)]
    struct Events {
        branch_protection_rule: Option<BranchProtectionRuleEvent>,
        check_run: Option<CheckRunEvent>,
        check_suite: Option<CheckSuiteEvent>,
        create: bool,
        delete: bool,
        deployment: bool,
        deployment_status: bool,
        discussion: Option<DiscussionEvent>,
        discussion_comment: Option<DiscussionCommentEvent>,
        fork: bool,
        gollum: bool,
        image_version: Option<ImageVersionEvent>,
        issue_comment: Option<IssueCommentEvent>,
        issues: Option<IssuesEvent>,
        label: Option<LabelEvent>,
        merge_group: Option<MergeGroupEvent>,
        milestone: Option<MilestoneEvent>,
        page_build: bool,
        public: bool,
        pull_request: Option<PullRequestEvent>,
        pull_request_review: Option<PullRequestReviewEvent>,
        pull_request_review_comment: Option<PullRequestReviewCommentEvent>,
        pull_request_target: Option<PullRequestEvent>,
        push: Option<PushEvent>,
        registry_package: Option<RegistryPackageEvent>,
        release: Option<ReleaseEvent>,
        schedule: Option<ScheduleEvent>,
        status: bool,
        watch: Option<WatchEvent>,
        workflow_call: Option<WorkflowCallEvent>,
        workflow_dispatch: Option<WorkflowDispatchEvent>,
        workflow_run: Option<WorkflowRunEvent>,
    }
    #[pymethods]
    impl Events {
        #[new]
        #[pyo3(signature = (*, branch_protection_rule=None, check_run=None, check_suite=None, create=false, delete=false, deployment=false, deployment_status=false, discussion=None, discussion_comment=None, fork=false, gollum=false, image_version=None, issue_comment=None, issues=None, label=None, merge_group=None, milestone=None, page_build=false, public=false, pull_request=None, pull_request_review=None, pull_request_review_comment=None, pull_request_target=None, push=None, registry_package=None, release=None, schedule=None, status=false, watch=None, workflow_call=None, workflow_dispatch=None, workflow_run=None))]
        fn new(
            branch_protection_rule: Option<BranchProtectionRuleEvent>,
            check_run: Option<CheckRunEvent>,
            check_suite: Option<CheckSuiteEvent>,
            create: bool,
            delete: bool,
            deployment: bool,
            deployment_status: bool,
            discussion: Option<DiscussionEvent>,
            discussion_comment: Option<DiscussionCommentEvent>,
            fork: bool,
            gollum: bool,
            image_version: Option<ImageVersionEvent>,
            issue_comment: Option<IssueCommentEvent>,
            issues: Option<IssuesEvent>,
            label: Option<LabelEvent>,
            merge_group: Option<MergeGroupEvent>,
            milestone: Option<MilestoneEvent>,
            page_build: bool,
            public: bool,
            pull_request: Option<PullRequestEvent>,
            pull_request_review: Option<PullRequestReviewEvent>,
            pull_request_review_comment: Option<PullRequestReviewCommentEvent>,
            pull_request_target: Option<PullRequestEvent>,
            push: Option<PushEvent>,
            registry_package: Option<RegistryPackageEvent>,
            release: Option<ReleaseEvent>,
            schedule: Option<ScheduleEvent>,
            status: bool,
            watch: Option<WatchEvent>,
            workflow_call: Option<WorkflowCallEvent>,
            workflow_dispatch: Option<WorkflowDispatchEvent>,
            workflow_run: Option<WorkflowRunEvent>,
        ) -> Self {
            Self {
                branch_protection_rule,
                check_run,
                check_suite,
                create,
                delete,
                deployment,
                deployment_status,
                discussion,
                discussion_comment,
                fork,
                gollum,
                image_version,
                issue_comment,
                issues,
                label,
                merge_group,
                milestone,
                page_build,
                public,
                pull_request,
                pull_request_review,
                pull_request_review_comment,
                pull_request_target,
                push,
                registry_package,
                release,
                schedule,
                status,
                watch,
                workflow_call,
                workflow_dispatch,
                workflow_run,
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.maybe_as_yaml_string()
        }
    }
    impl MaybeYamlable for &Events {
        fn maybe_as_yaml(&self) -> Option<Yaml> {
            let mut configured = Hash::new();
            let mut simple_names: Vec<&str> = Vec::new();

            if let Some(event) = &self.branch_protection_rule {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("branch_protection_rule", yaml);
                } else {
                    simple_names.push("branch_protection_rule");
                }
            }
            if let Some(event) = &self.check_run {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("check_run", yaml);
                } else {
                    simple_names.push("check_run");
                }
            }
            if let Some(event) = &self.check_suite {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("check_suite", yaml);
                } else {
                    simple_names.push("check_suite");
                }
            }
            if let Some(event) = &self.discussion {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("discussion", yaml);
                } else {
                    simple_names.push("discussion");
                }
            }
            if let Some(event) = &self.discussion_comment {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("discussion_comment", yaml);
                } else {
                    simple_names.push("discussion_comment");
                }
            }

            if let Some(event) = &self.image_version {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("image_version", yaml);
                } else {
                    simple_names.push("image_version");
                }
            }
            if let Some(event) = &self.issue_comment {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("issue_comment", yaml);
                } else {
                    simple_names.push("issue_comment");
                }
            }
            if let Some(event) = &self.issues {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("issues", yaml);
                } else {
                    simple_names.push("issues");
                }
            }
            if let Some(event) = &self.label {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("label", yaml);
                } else {
                    simple_names.push("label");
                }
            }
            if let Some(event) = &self.merge_group {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("merge_group", yaml);
                } else {
                    simple_names.push("merge_group");
                }
            }
            if let Some(event) = &self.milestone {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("milestone", yaml);
                } else {
                    simple_names.push("milestone");
                }
            }
            if let Some(event) = &self.pull_request {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("pull_request", yaml);
                } else {
                    simple_names.push("pull_request");
                }
            }
            if let Some(event) = &self.pull_request_review {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("pull_request_review", yaml);
                } else {
                    simple_names.push("pull_request_review");
                }
            }
            if let Some(event) = &self.pull_request_review_comment {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("pull_request_review_comment", yaml);
                } else {
                    simple_names.push("pull_request_review_comment");
                }
            }
            if let Some(event) = &self.pull_request_target {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("pull_request_target", yaml);
                } else {
                    simple_names.push("pull_request_target");
                }
            }
            if let Some(event) = &self.push {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("push", yaml);
                } else {
                    simple_names.push("push");
                }
            }
            if let Some(event) = &self.registry_package {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("registry_package", yaml);
                } else {
                    simple_names.push("registry_package");
                }
            }
            if let Some(event) = &self.release {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("release", yaml);
                } else {
                    simple_names.push("release");
                }
            }
            if let Some(event) = &self.schedule {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("schedule", yaml);
                } else {
                    simple_names.push("schedule");
                }
            }
            if let Some(event) = &self.watch {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("watch", yaml);
                } else {
                    simple_names.push("watch");
                }
            }
            if let Some(event) = &self.workflow_call {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("workflow_call", yaml);
                } else {
                    simple_names.push("workflow_call");
                }
            }
            if let Some(event) = &self.workflow_dispatch {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("workflow_dispatch", yaml);
                } else {
                    simple_names.push("workflow_dispatch");
                }
            }
            if let Some(event) = &self.workflow_run {
                if let Some(yaml) = event.maybe_as_yaml() {
                    configured.insert_yaml("workflow_run", yaml);
                } else {
                    simple_names.push("workflow_run");
                }
            }

            if self.create {
                simple_names.push("create");
            }
            if self.delete {
                simple_names.push("delete");
            }
            if self.deployment {
                simple_names.push("deployment");
            }
            if self.deployment_status {
                simple_names.push("deployment_status");
            }
            if self.fork {
                simple_names.push("fork");
            }
            if self.gollum {
                simple_names.push("gollum");
            }
            if self.page_build {
                simple_names.push("page_build");
            }
            if self.public {
                simple_names.push("public");
            }
            if self.status {
                simple_names.push("status");
            }

            if configured.is_empty() {
                match simple_names.len() {
                    0 => None,
                    1 => Some(simple_names[0].as_yaml()),
                    _ => {
                        let mut arr = Array::new();
                        for name in simple_names {
                            arr.push_yaml(name);
                        }
                        Some(Yaml::Array(arr))
                    }
                }
            } else {
                for name in simple_names {
                    configured.insert_yaml(name, Yaml::Null);
                }
                Some(Yaml::Hash(configured))
            }
        }
    }

    #[pyclass]
    struct Workflow {
        name: Option<String>,
        run_name: Option<StringLike>,
        on: Events,
        permissions: Option<Permissions>,
        env: Option<PyMap<String, StringLike>>,
        defaults: Option<Defaults>,
        concurrency: Option<Concurrency>,
        jobs: PyMap<String, Job>,
    }
    #[pymethods]
    impl Workflow {
        #[new]
        #[pyo3(signature = (*, jobs, on, name = None, run_name = None, permissions = None, env = None, defaults = None, concurrency = None))]
        fn new(
            jobs: PyMap<String, Job>,
            on: Events,
            name: Option<String>,
            run_name: Option<StringLike>,
            permissions: Option<Permissions>,
            env: Option<PyMap<String, StringLike>>,
            defaults: Option<Defaults>,
            concurrency: Option<Concurrency>,
        ) -> Self {
            Self {
                name,
                run_name,
                on,
                permissions,
                env,
                defaults,
                concurrency,
                jobs,
            }
        }

        #[pyo3(signature = (path, *, overwrite = true))]
        fn dump(&self, path: Bound<PyAny>, overwrite: bool) -> PyResult<()> {
            if let Ok(p) = path.extract::<PathBuf>() {
                self.write_to_file(p, overwrite)
            } else if let Ok(s) = path.extract::<String>() {
                self.write_to_file(s, overwrite)
            } else {
                Err(PyValueError::new_err("Invalid path"))
            }
        }

        fn __str__(&self) -> PyResult<String> {
            self.as_yaml_string()
        }
    }
    impl Yamlable for &Workflow {
        fn as_yaml(&self) -> Yaml {
            let mut out = Hash::new();
            out.insert_yaml_opt("name", &self.name);
            out.insert_yaml_opt("run-name", &self.run_name);
            out.insert_yaml_opt("on", (&self.on).maybe_as_yaml());
            out.insert_yaml_opt("permissions", &self.permissions);
            out.insert_yaml_opt("env", &self.env);
            if let Some(defaults) = &self.defaults {
                out.insert_yaml_opt("defaults", defaults.maybe_as_yaml());
            }
            out.insert_yaml_opt("concurrency", &self.concurrency);
            out.insert_yaml("jobs", &self.jobs);
            Yaml::Hash(out)
        }
    }
}
