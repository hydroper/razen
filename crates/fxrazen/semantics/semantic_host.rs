use crate::ns::*;

pub struct SemanticHost {
    pub(crate) arena: ThingyArena,

    project_path: Option<String>,
    env_cache: RefCell<Option<Rc<HashMap<String, String>>>>,

    unused_things: Rc<RefCell<Vec<Thingy>>>,

    pub(crate) explicit_namespaces: RefCell<HashMap<String, Thingy>>,
    pub(crate) user_namespaces: RefCell<HashMap<String, Thingy>>,
    pub(crate) qnames: RefCell<HashMap<Thingy, HashMap<String, QName>>>,
    invalidation_thingy: Thingy,
    unresolved_thingy: Thingy,
    pub(crate) top_level_package: Thingy,
    as3_vec_package: RefCell<Option<Thingy>>,
    any_type: Thingy,
    void_type: Thingy,
    object_type: RefCell<Option<Thingy>>,
    boolean_type: RefCell<Option<Thingy>>,
    number_type: RefCell<Option<Thingy>>,
    int_type: RefCell<Option<Thingy>>,
    uint_type: RefCell<Option<Thingy>>,
    float_type: RefCell<Option<Thingy>>,
    string_type: RefCell<Option<Thingy>>,
    array_type: RefCell<Option<Thingy>>,
    namespace_type: RefCell<Option<Thingy>>,
    function_type: RefCell<Option<Thingy>>,
    class_type: RefCell<Option<Thingy>>,
    xml_type: RefCell<Option<Thingy>>,
    xml_list_type: RefCell<Option<Thingy>>,
    vector_type: RefCell<Option<Thingy>>,

    meta_prop: Thingy,
    meta_env_prop: Thingy,

    non_null_primitive_types: RefCell<Option<Rc<Vec<Thingy>>>>,
    numeric_types: RefCell<Option<Rc<Vec<Thingy>>>>,
    floating_point_types: RefCell<Option<Rc<Vec<Thingy>>>>,
    integer_types: RefCell<Option<Rc<Vec<Thingy>>>>,
    pub(crate) types_after_sub: RefCell<HashMap<Thingy, Vec<Thingy>>>,
    pub(crate) function_types: RefCell<HashMap<usize, Vec<Thingy>>>,
    pub(crate) tuple_types: RefCell<HashMap<usize, Vec<Thingy>>>,
    pub(crate) nullable_types: RefCell<HashMap<Thingy, Thingy>>,
    pub(crate) non_nullable_types: RefCell<HashMap<Thingy, Thingy>>,
    // Slots after indirect type substitution (variable, method, and virtual slots).
    pub(crate) vasub: RefCell<HashMap<Thingy, HashMap<SharedArray<Thingy>, Vec<Thingy>>>>,
    pub(crate) visub: RefCell<HashMap<Thingy, HashMap<SharedArray<Thingy>, Vec<Thingy>>>>,
    pub(crate) mssub: RefCell<HashMap<Thingy, HashMap<SharedArray<Thingy>, Vec<Thingy>>>>,
}

impl SemanticHost {
    pub fn new(options: SemanticHostOptions) -> Self {
        let arena = ThingyArena::new();
        let explicit_namespaces = RefCell::new(HashMap::new());
        let user_namespaces = RefCell::new(HashMap::new());
        let qnames = RefCell::new(HashMap::new());
        let any_type: Thingy = AnyType::new(&arena).into();
        let void_type: Thingy = VoidType::new(&arena).into();
        let invalidation_thingy: Thingy = InvalidationThingy::new(&arena).into();
        let unresolved_thingy: Thingy = UnresolvedThingy::new(&arena).into();
        let top_level_package = Package::new(&arena, "".into());
        let meta_prop: Thingy = MetaProperty::new(&arena, &any_type).into();
        let meta_env_prop: Thingy = MetaEnvProperty::new(&arena, &any_type).into();
        let host = Self {
            arena,
            project_path: options.project_path.clone(),
            env_cache: RefCell::new(None),
            explicit_namespaces,
            user_namespaces,
            qnames,
            top_level_package: top_level_package.clone().into(),
            as3_vec_package: RefCell::new(None),
            invalidation_thingy,
            unresolved_thingy,

            unused_things: Rc::new(RefCell::new(vec![])),

            meta_prop,
            meta_env_prop,

            any_type,
            void_type,
            object_type: RefCell::new(None),
            boolean_type: RefCell::new(None),
            number_type: RefCell::new(None),
            int_type: RefCell::new(None),
            uint_type: RefCell::new(None),
            float_type: RefCell::new(None),
            string_type: RefCell::new(None),
            array_type: RefCell::new(None),
            namespace_type: RefCell::new(None),
            function_type: RefCell::new(None),
            class_type: RefCell::new(None),
            xml_type: RefCell::new(None),
            xml_list_type: RefCell::new(None),
            vector_type: RefCell::new(None),

            non_null_primitive_types: RefCell::new(None),
            numeric_types: RefCell::new(None),
            floating_point_types: RefCell::new(None),
            integer_types: RefCell::new(None),
            types_after_sub: RefCell::new(HashMap::new()),
            function_types: RefCell::new(HashMap::new()),
            tuple_types: RefCell::new(HashMap::new()),
            nullable_types: RefCell::new(HashMap::new()),
            non_nullable_types: RefCell::new(HashMap::new()),
            vasub: RefCell::new(HashMap::new()),
            visub: RefCell::new(HashMap::new()),
            mssub: RefCell::new(HashMap::new()),
        };

        // Initialize top level namespaces
        top_level_package.set_public_ns(Some(host.factory().create_public_ns(Some(top_level_package.clone().into()))));
        top_level_package.set_internal_ns(Some(host.factory().create_internal_ns(Some(top_level_package.clone().into()))));

        host
    }

    #[inline(always)]
    pub fn factory(&self) -> ThingyFactory {
        ThingyFactory(self)
    }

    pub fn top_level_package(&self) -> Thingy {
        self.top_level_package.clone()
    }

    pub fn as3_vec_package(&self) -> Thingy {
        if let Some(p) = self.as3_vec_package.borrow().as_ref() {
            return p.clone();
        }
        let p = self.factory().create_package(["__AS3__", "vec"]);
        self.as3_vec_package.replace(Some(p.clone()));
        p
    }

    pub fn invalidation_thingy(&self) -> Thingy {
        self.invalidation_thingy.clone()
    }

    pub fn unresolved_thingy(&self) -> Thingy {
        self.unresolved_thingy.clone()
    }

    pub fn any_type(&self) -> Thingy {
        self.any_type.clone()
    }

    pub fn void_type(&self) -> Thingy {
        self.void_type.clone()
    }

    pub fn meta_property(&self) -> Thingy {
        self.meta_prop.clone()
    }

    pub fn meta_env_property(&self) -> Thingy {
        self.meta_env_prop.clone()
    }

    global_lookup!(object_type, "Object");
    global_lookup!(boolean_type, "Boolean");
    global_lookup!(number_type, "Number");
    global_lookup!(int_type, "int");
    global_lookup!(uint_type, "uint");
    global_lookup!(float_type, "float");
    global_lookup!(string_type, "String");
    global_lookup!(array_type, "Array");
    global_lookup!(namespace_type, "Namespace");
    global_lookup!(function_type, "Function");
    global_lookup!(class_type, "Class");
    global_lookup!(xml_type, "XML");
    global_lookup!(xml_list_type, "XMLList");

    /// Retrieves `__AS3__.vec.Vector`, a possibly unresolved thing.
    pub fn vector_type(&self) -> Thingy {
        if let Some(r) = self.vector_type.borrow().as_ref() {
            return r.clone();
        }
        let pckg = self.as3_vec_package();
        if let Some(r) = pckg.properties(self).get(&self.factory().create_qname(&pckg.public_ns().unwrap().into(), "Vector".to_owned())) {
            self.vector_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved_thingy()
        }
    }

    /// Returns the set of primitive types that do not contain `null`,
    /// such as `Boolean`, `Number`, `int`, `uint`, and `float`.
    /// `String` is never included in the resulting set, as it
    /// includes the `null` value.
    pub fn non_null_primitive_types(&self) -> Result<Rc<Vec<Thingy>>, DeferError> {
        if let Some(r) = self.non_null_primitive_types.borrow().as_ref() {
            return Ok(r.clone());
        }
        let r = Rc::new(vec![
            self.boolean_type().defer()?,
            self.number_type().defer()?,
            self.int_type().defer()?,
            self.uint_type().defer()?,
            self.float_type().defer()?,
        ]);
        self.non_null_primitive_types.replace(Some(r.clone()));
        Ok(r)
    }

    pub fn numeric_types(&self) -> Result<Rc<Vec<Thingy>>, DeferError> {
        if let Some(r) = self.numeric_types.borrow().as_ref() {
            return Ok(r.clone());
        }
        let r = Rc::new(vec![
            self.number_type().defer()?,
            self.int_type().defer()?,
            self.uint_type().defer()?,
            self.float_type().defer()?,
        ]);
        self.numeric_types.replace(Some(r.clone()));
        Ok(r)
    }

    pub fn floating_point_types(&self) -> Result<Rc<Vec<Thingy>>, DeferError> {
        if let Some(r) = self.floating_point_types.borrow().as_ref() {
            return Ok(r.clone());
        }
        let r = Rc::new(vec![
            self.number_type().defer()?,
            self.float_type().defer()?,
        ]);
        self.floating_point_types.replace(Some(r.clone()));
        Ok(r)
    }

    pub fn integer_types(&self) -> Result<Rc<Vec<Thingy>>, DeferError> {
        if let Some(r) = self.integer_types.borrow().as_ref() {
            return Ok(r.clone());
        }
        let r = Rc::new(vec![
            self.int_type().defer()?,
            self.uint_type().defer()?,
        ]);
        self.integer_types.replace(Some(r.clone()));
        Ok(r)
    }

    /// Preloads environment variables from the main project's `.env` file
    /// using the DotEnv file format.
    pub fn env(&self) -> Rc<HashMap<String, String>> {
        if let Some(env) = self.env_cache.borrow().as_ref() {
            return env.clone();
        }
        let mut r = HashMap::<String, String>::new();
        if let Some(project_path) = self.project_path.as_ref() {
            if let Ok(iterator) = dotenvy::from_path_iter(project_path) {
                for item in iterator {
                    if let Ok((key, value)) = item {
                        r.insert(key, value);
                    }
                }
            }
        }
        let r = Rc::new(r);
        self.env_cache.replace(Some(r.clone()));
        r
    }

    pub(crate) fn unused_things(&self) -> std::cell::Ref<Vec<Thingy>> {
        self.unused_things.borrow()
    }

    pub(crate) fn add_unused_thing(&self, thing: &Thingy) {
        self.unused_things.borrow_mut().push(thing.clone());
    }

    pub(crate) fn remove_unused_thing(&self, thing: &Thingy) {
        let mut i = 0usize;
        let mut things = self.unused_things.borrow_mut();
        for t1 in things.iter() {
            if thing == t1 {
                things.remove(i);
                break;
            }
            i += 1;
        }
    }
}

#[derive(Clone)]
pub struct SemanticHostOptions {
    /// The directory path of the main project being compiled,
    /// used for the `import.meta.env.EXAMPLE` accessors.
    pub project_path: Option<String>,
}

impl Default for SemanticHostOptions {
    fn default() -> Self {
        Self {
            project_path: None,
        }
    }
}

macro global_lookup {
    ($field:ident, $as3name:expr) => {
        /// Retrieves a possibly unresolved thing.
        pub fn $field(&self) -> Thingy {
            if let Some(r) = self.$field.borrow().as_ref() {
                return r.clone();
            }
            if let Some(r) = self.top_level_package.properties(self).get(&self.factory().create_qname(&self.top_level_package.public_ns().unwrap().into(), $as3name.to_owned())) {
                self.$field.replace(Some(r.clone()));
                r
            } else {
                self.unresolved_thingy()
            }
        }
    },
}