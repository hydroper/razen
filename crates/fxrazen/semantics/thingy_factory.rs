use crate::ns::*;

pub struct ThingyFactory<'a>(pub(crate) &'a SemanticHost);

impl<'a> ThingyFactory<'a> {
    pub fn create_public_ns(&self, parent: Option<Thingy>) -> Thingy {
        SystemNamespace::new(&self.0.arena, SystemNamespaceKind::Public, parent).into()
    }

    pub fn create_private_ns(&self, parent: Option<Thingy>) -> Thingy {
        SystemNamespace::new(&self.0.arena, SystemNamespaceKind::Private, parent).into()
    }

    pub fn create_protected_ns(&self, parent: Option<Thingy>) -> Thingy {
        SystemNamespace::new(&self.0.arena, SystemNamespaceKind::Protected, parent).into()
    }

    pub fn create_static_protected_ns(&self, parent: Option<Thingy>) -> Thingy {
        SystemNamespace::new(&self.0.arena, SystemNamespaceKind::StaticProtected, parent).into()
    }

    pub fn create_internal_ns(&self, parent: Option<Thingy>) -> Thingy {
        SystemNamespace::new(&self.0.arena, SystemNamespaceKind::Internal, parent).into()
    }

    pub fn create_explicit_ns(&self, uri: String) -> Thingy {
        let mut mappings = self.0.explicit_namespaces.borrow_mut();
        if let Some(ns) = mappings.get(&uri) {
            return ns.clone();
        }
        let ns: Thingy = ExplicitNamespace::new(&self.0.arena, uri.clone()).into();
        mappings.insert(uri, ns.clone());
        ns
    }

    pub fn create_user_ns(&self, uri: String) -> Thingy {
        let mut mappings = self.0.user_namespaces.borrow_mut();
        if let Some(ns) = mappings.get(&uri) {
            return ns.clone();
        }
        let ns: Thingy = UserNamespace::new(&self.0.arena, uri.clone()).into();
        mappings.insert(uri, ns.clone());
        ns
    }

    pub fn create_qname(&self, namespace: &Thingy, local_name: String) -> QName {
        let mut ns_mappings = self.0.qnames.borrow_mut();
        if let Some(qn_mappings) = ns_mappings.get_mut(namespace) {
            if let Some(qn) = qn_mappings.get(&local_name) {
                return qn.clone();
            }
            let qn = QName(Rc::new(QName1 {
                m_namespace: namespace.clone(),
                m_local_name: local_name.clone(),
            }));
            qn_mappings.insert(local_name, qn.clone());
            return qn;
        }
        let qn = QName(Rc::new(QName1 {
            m_namespace: namespace.clone(),
            m_local_name: local_name.clone(),
        }));
        let mut qn_mappings = HashMap::new();
        qn_mappings.insert(local_name, qn.clone());
        ns_mappings.insert(namespace.clone(), qn_mappings);
        qn
    }

    /// Interns a package from a fully qualified name.
    ///
    /// # Example
    ///
    /// ```ignore
    /// assert_eq!(host.factory().create_package(["foo", "bar"]).fully_qualified_name(), "foo.bar");
    /// ```
    pub fn create_package<'b>(&self, name: impl IntoIterator<Item = &'b str>) -> Thingy {
        self.create_package_1(&name.into_iter().collect())
    }

    fn create_package_1(&self, name: &Vec<&str>) -> Thingy {
        let mut result: Thingy = self.0.top_level_package.clone();
        for name_1 in name {
            let name_1 = (*name_1).to_owned();
            let result_1 = result.subpackages().get(&name_1);
            if let Some(result_1) = result_1 {
                result = result_1;
            } else {
                let result_1 = Package::new(&self.0.arena, name_1.clone());
                result_1.set_parent(Some(result.clone().into()));

                // Assign namespaces
                result_1.set_public_ns(Some(self.create_public_ns(Some(result_1.clone().into()))));
                result_1.set_internal_ns(Some(self.create_internal_ns(Some(result_1.clone().into()))));

                result.subpackages().set(name_1, result_1.clone().into());
                result = result_1.into();
            }
        }
        result
    }

    pub fn create_alias(&self, name: QName, alias_of: Thingy) -> Thingy {
        Alias::new(&self.0.arena, name, alias_of).into()
    }

    /// # Parameters
    /// 
    /// - `ns_for_prototype`: The namespace used for the `prototype` property. Either
    ///   `public` or `internal`.
    pub fn create_class_type(&self, name: QName, ns_for_prototype: &Thingy) -> Thingy {
        let r = ClassType::new(&self.0.arena, name);
        r.set_private_ns(Some(self.create_private_ns(Some(r.clone().into()))));
        r.set_protected_ns(Some(self.create_protected_ns(Some(r.clone().into()))));
        r.set_static_protected_ns(Some(self.create_static_protected_ns(Some(r.clone().into()))));

        // "static const prototype: *;"
        let prototype_name = self.create_qname(&ns_for_prototype, "prototype".into());
        let prototype_slot = self.create_variable_slot(&prototype_name, true, &self.0.any_type());
        r.properties(self.0).set(prototype_name.clone(), prototype_slot);

        r.into()
    }

    pub fn create_enum_type(&self, name: QName, ns_for_prototype: &Thingy) -> Thingy {
        let r = EnumType::new(&self.0.arena, name);
        r.set_private_ns(Some(self.create_private_ns(Some(r.clone().into()))));

        // "static const prototype: *;"
        let prototype_name = self.create_qname(&ns_for_prototype, "prototype".into());
        let prototype_slot = self.create_variable_slot(&prototype_name, true, &self.0.any_type());
        r.properties(self.0).set(prototype_name.clone(), prototype_slot);

        r.into()
    }

    pub fn create_interface_type(&self, name: QName) -> Thingy {
        let r = InterfaceType::new(&self.0.arena, name);
        r.into()
    }

    /// Interns type after substitution.
    pub fn create_type_after_substitution(&self, origin: &Thingy, substitute_types: &SharedArray<Thingy>) -> Thingy {
        // Verify parameter count
        let params = origin.type_params().unwrap();
        let param_count = params.length();
        assert_eq!(substitute_types.length(), param_count);

        let mut tas_list = self.0.types_after_sub.borrow_mut();

        let mut list = tas_list.get(&origin);
        let empty_list = vec![];
        if list.is_none() {
            list = Some(&empty_list);
            tas_list.insert(origin.clone(), vec![]);
        }
        'tas: for tas in list.unwrap() {
            let mut substitute_types_1 = substitute_types.iter();
            let substitute_types_2 = tas.substitute_types();
            let mut substitute_types_2 = substitute_types_2.iter();
            while let Some(substitute_type_1) = substitute_types_1.next() {
                let substitute_type_2 = substitute_types_2.next().unwrap();
                if substitute_type_1 != substitute_type_2 {
                    continue 'tas;
                }
            }
            return tas.clone();
        }

        let tas = TypeAfterSubstitution::new(&self.0.arena, origin.clone(), substitute_types.clone());
        let list = tas_list.get_mut(&origin).unwrap();
        list.push(tas.clone().into());

        tas.into()
    }

    /// Interns a tuple type.
    pub fn create_tuple_type(&self, element_types: Vec<Thingy>) -> Thingy {
        let element_count = element_types.len();
        let mut tuple_types = self.0.tuple_types.borrow_mut();
        let mut collection = tuple_types.get_mut(&element_count);
        let mut empty_collection = vec![];
        if collection.is_none() {
            collection = Some(&mut empty_collection);
            tuple_types.insert(element_count, vec![]);
        }
        'tt: for tt in collection.unwrap() {
            let mut element_types_1 = element_types.iter();
            let element_types_2 = tt.element_types();
            let mut element_types_2 = element_types_2.iter();
            while let Some(e_1) = element_types_1.next() {
                let e_2 = element_types_2.next().unwrap();
                if e_1 != &e_2 {
                    continue 'tt;
                }
            }
            return tt.clone();
        }
        let tt = TupleType::new(&self.0.arena, SharedArray::from(element_types));

        let collection = tuple_types.get_mut(&element_count);
        collection.unwrap().push(tt.clone().into());

        tt.into()
    }

    /// Interns a function type.
    pub fn create_function_type(&self, params: Vec<Rc<SemanticFunctionTypeParameter>>, result_type: Thingy) -> Thingy {
        let param_count = params.len();
        let mut function_types = self.0.function_types.borrow_mut();
        let mut collection = function_types.get_mut(&param_count);
        let mut empty_collection = vec![];
        if collection.is_none() {
            collection = Some(&mut empty_collection);
            function_types.insert(params.len(), vec![]);
        }
        'ft: for ft in collection.unwrap() {
            if result_type != ft.result_type() {
                continue 'ft;
            }
            let mut params_1 = params.iter();
            let params_2 = ft.params();
            let mut params_2 = params_2.iter();
            while let Some(param_1) = params_1.next() {
                let param_2 = params_2.next().unwrap();
                if !(param_1.kind == param_2.kind && && param_1.static_type == &&param_2.static_type) {
                    continue 'ft;
                }
            }
            return ft.clone();
        }
        let ft = FunctionType::new(&self.0.arena, SharedArray::from(params), result_type);

        let collection = function_types.get_mut(&param_count);
        collection.unwrap().push(ft.clone().into());

        ft.into()
    }

    /// Interns a nullable type.
    pub fn create_nullable_type(&self, base: &Thingy) -> Thingy {
        if base == &self.0.any_type() || base.is::<NullableType>() {
            return base.clone();
        }
        if base.is::<NonNullableType>() {
            return base.base();
        }
        let mut m = self.0.nullable_types.borrow_mut();
        let nt = m.get(base);
        if let Some(nt) = nt {
            return nt.clone();
        }
        let nt = NullableType::new(&self.0.arena, base.clone());
        m.insert(base.clone(), nt.clone().into());
        nt.into()
    }

    /// Interns a non nullable type.
    pub fn create_non_nullable_type(&self, base: &Thingy) -> Thingy {
        if base == &self.0.any_type() || base.is::<NonNullableType>() {
            return base.clone();
        }
        let mut m = self.0.non_nullable_types.borrow_mut();
        let nt = m.get(base);
        if let Some(nt) = nt {
            return nt.clone();
        }
        let nt = NonNullableType::new(&self.0.arena, base.clone());
        m.insert(base.clone(), nt.clone().into());
        nt.into()
    }

    pub fn create_type_parameter_type(&self, name: &QName) -> Thingy {
        TypeParameterType::new(&self.0.arena, name.clone()).into()
    }

    pub fn create_variable_slot(&self, name: &QName, read_only: bool, static_type: &Thingy) -> Thingy {
        OriginalVariableSlot::new(&self.0.arena, name, read_only, static_type).into()
    }

    /// Interns a variable slot after indirect substitution.
    pub fn create_variable_slot_after_substitution(&self, origin: &Thingy, indirect_type_params: &SharedArray<Thingy>, indirect_substitute_types: &SharedArray<Thingy>) -> Thingy {
        // Verify parameter count
        assert_eq!(indirect_type_params.length(), indirect_substitute_types.length());

        let mut vasub_list = self.0.vasub.borrow_mut();

        let mut base_list = vasub_list.get_mut(origin);
        let mut empty_base_list = HashMap::<SharedArray<Thingy>, Vec<Thingy>>::new();
        if base_list.is_none() {
            base_list = Some(&mut empty_base_list);
            vasub_list.insert(origin.clone(), HashMap::new());
        }
        let base_list = base_list.unwrap();

        let mut list = base_list.get(indirect_type_params);
        let empty_list = vec![];
        if list.is_none() {
            list = Some(&empty_list);
            base_list.insert(indirect_type_params.clone(), vec![]);
        }
        'vasub: for vasub in list.unwrap() {
            let mut substitute_types_1 = indirect_substitute_types.iter();
            let substitute_types_2 = vasub.indirect_substitute_types();
            let mut substitute_types_2 = substitute_types_2.iter();
            while let Some(substitute_type_1) = substitute_types_1.next() {
                let substitute_type_2 = substitute_types_2.next().unwrap();
                if substitute_type_1 != substitute_type_2 {
                    continue 'vasub;
                }
            }
            return vasub.clone();
        }

        let vasub = VariableSlotAfterSubstitution::new(
            &self.0.arena,
            &origin,
            &indirect_type_params,
            &indirect_substitute_types.clone());

        let list = vasub_list.get_mut(origin).unwrap().get_mut(&indirect_type_params).unwrap();
        list.push(vasub.clone().into());

        vasub.into()
    }

    pub fn create_virtual_slot(&self, name: &QName) -> Thingy {
        OriginalVirtualSlot::new(&self.0.arena, name).into()
    }

    /// Interns a virtual slot after indirect substitution.
    pub fn create_virtual_slot_after_substitution(&self, origin: &Thingy, indirect_type_params: &SharedArray<Thingy>, indirect_substitute_types: &SharedArray<Thingy>) -> Thingy {
        // Verify parameter count
        assert_eq!(indirect_type_params.length(), indirect_substitute_types.length());

        let mut visub_list = self.0.visub.borrow_mut();

        let mut base_list = visub_list.get_mut(origin);
        let mut empty_base_list = HashMap::<SharedArray<Thingy>, Vec<Thingy>>::new();
        if base_list.is_none() {
            base_list = Some(&mut empty_base_list);
            visub_list.insert(origin.clone(), HashMap::new());
        }
        let base_list = base_list.unwrap();

        let mut list = base_list.get(indirect_type_params);
        let empty_list = vec![];
        if list.is_none() {
            list = Some(&empty_list);
            base_list.insert(indirect_type_params.clone(), vec![]);
        }
        'visub: for visub in list.unwrap() {
            let mut substitute_types_1 = indirect_substitute_types.iter();
            let substitute_types_2 = visub.indirect_substitute_types();
            let mut substitute_types_2 = substitute_types_2.iter();
            while let Some(substitute_type_1) = substitute_types_1.next() {
                let substitute_type_2 = substitute_types_2.next().unwrap();
                if substitute_type_1 != substitute_type_2 {
                    continue 'visub;
                }
            }
            return visub.clone();
        }

        let visub = VirtualSlotAfterSubstitution::new(
            &self.0.arena,
            &origin,
            &indirect_type_params,
            &indirect_substitute_types.clone());

        let list = visub_list.get_mut(origin).unwrap().get_mut(&indirect_type_params).unwrap();
        list.push(visub.clone().into());

        visub.into()
    }

    pub fn create_method_slot(&self, name: &QName, signature: &Thingy) -> Thingy {
        OriginalMethodSlot::new(&self.0.arena, name, signature).into()
    }

    /// Interns a method slot after indirect substitution.
    pub fn create_method_slot_after_substitution(&self, origin: &Thingy, indirect_type_params: &SharedArray<Thingy>, indirect_substitute_types: &SharedArray<Thingy>) -> Thingy {
        // Verify parameter count
        assert_eq!(indirect_type_params.length(), indirect_substitute_types.length());

        let mut mssub_list = self.0.mssub.borrow_mut();

        let mut base_list = mssub_list.get_mut(origin);
        let mut empty_base_list = HashMap::<SharedArray<Thingy>, Vec<Thingy>>::new();
        if base_list.is_none() {
            base_list = Some(&mut empty_base_list);
            mssub_list.insert(origin.clone(), HashMap::new());
        }
        let base_list = base_list.unwrap();

        let mut list = base_list.get(indirect_type_params);
        let empty_list = vec![];
        if list.is_none() {
            list = Some(&empty_list);
            base_list.insert(indirect_type_params.clone(), vec![]);
        }
        'mssub: for mssub in list.unwrap() {
            let mut substitute_types_1 = indirect_substitute_types.iter();
            let substitute_types_2 = mssub.indirect_substitute_types();
            let mut substitute_types_2 = substitute_types_2.iter();
            while let Some(substitute_type_1) = substitute_types_1.next() {
                let substitute_type_2 = substitute_types_2.next().unwrap();
                if substitute_type_1 != substitute_type_2 {
                    continue 'mssub;
                }
            }
            return mssub.clone();
        }

        let mssub = MethodSlotAfterSubstitution::new(
            &self.0.arena,
            &origin,
            &indirect_type_params,
            &indirect_substitute_types.clone());

        let list = mssub_list.get_mut(origin).unwrap().get_mut(&indirect_type_params).unwrap();
        list.push(mssub.clone().into());

        mssub.into()
    }

    pub fn create_scope(&self) -> Thingy {
        Scope::new(&self.0.arena).into()
    }

    pub fn create_with_scope(&self, object: &Thingy) -> Thingy {
        WithScope::new(&self.0.arena, object).into()
    }

    pub fn create_filter_scope(&self, base: &Thingy) -> Thingy {
        FilterScope::new(&self.0.arena, base).into()
    }

    pub fn create_activation(&self, of_method: &Thingy) -> Thingy {
        Activation::new(&self.0.arena, of_method).into()
    }

    pub fn create_class_scope(&self, class: &Thingy) -> Thingy {
        ClassScope::new(&self.0.arena, class).into()
    }

    pub fn create_enum_scope(&self, class: &Thingy) -> Thingy {
        EnumScope::new(&self.0.arena, class).into()
    }

    pub fn create_interface_scope(&self, itrfc: &Thingy) -> Thingy {
        InterfaceScope::new(&self.0.arena, itrfc).into()
    }

    pub fn create_package_scope(&self, pckg: &Thingy) -> Thingy {
        PackageScope::new(&self.0.arena, pckg).into()
    }

    pub fn create_value(&self, static_type: &Thingy) -> Thingy {
        Value::new(&self.0.arena, static_type).into()
    }

    pub fn create_package_property_import(&self, property: &Thingy, location: Option<Location>) -> Thingy {
        PackagePropertyImport::new(&self.0.arena, property, location, &self.0.any_type()).into()
    }

    pub fn create_package_wildcard_import(&self, package: &Thingy, location: Option<Location>) -> Thingy {
        PackageWildcardImport::new(&self.0.arena, package, location, &self.0.any_type()).into()
    }

    pub fn create_package_recursive_import(&self, package: &Thingy, location: Option<Location>) -> Thingy {
        PackageRecursiveImport::new(&self.0.arena, package, location, &self.0.any_type()).into()
    }

    pub fn create_undefined_constant(&self, static_type: &Thingy) -> Thingy {
        UndefinedConstant::new(&self.0.arena, static_type).into()
    }

    pub fn create_null_constant(&self, static_type: &Thingy) -> Thingy {
        NullConstant::new(&self.0.arena, static_type).into()
    }

    pub fn create_namespace_constant(&self, referenced_ns: &Thingy) -> Result<Thingy, DeferError> {
        Ok(NamespaceConstant::new(&self.0.arena, referenced_ns, &self.0.namespace_type().defer()?).into())
    }

    pub fn create_number_constant(&self, value: NumberVariant, static_type: &Thingy) -> Thingy {
        NumberConstant::new(&self.0.arena, value, static_type).into()
    }

    pub fn create_string_constant(&self, value: String, static_type: &Thingy) -> Thingy {
        StringConstant::new(&self.0.arena, value, static_type).into()
    }

    pub fn create_boolean_constant(&self, value: bool, static_type: &Thingy) -> Thingy {
        BooleanConstant::new(&self.0.arena, value, static_type).into()
    }

    pub fn create_this_object(&self, static_type: &Thingy) -> Thingy {
        ThisObject::new(&self.0.arena, static_type).into()
    }

    pub fn create_type_as_reference_value(&self, referenced_type: &Thingy) -> Result<Thingy, DeferError> {
        Ok(TypeAsReferenceValue::new(&self.0.arena, referenced_type, &self.0.class_type().defer()?).into())
    }

    pub fn create_xml_reference_value(&self, base: &Thingy, qualifier: Option<Thingy>, key: &Thingy) -> Thingy {
        XmlReferenceValue::new(&self.0.arena, base, qualifier, key, &self.0.any_type()).into()
    }

    pub fn create_dynamic_reference_value(&self, base: &Thingy, qualifier: Option<Thingy>, key: &Thingy) -> Thingy {
        DynamicReferenceValue::new(&self.0.arena, base, qualifier, key, &self.0.any_type()).into()
    }

    pub fn create_array_element_reference_value(&self, base: &Thingy, key: &Thingy) -> Result<Thingy, DeferError> {
        let st = base.static_type(self.0).defer()?.escape_of_non_nullable().array_element_type(self.0)?.unwrap();
        Ok(ArrayElementReferenceValue::new(&self.0.arena, base, key, &st).into())
    }

    pub fn create_vector_element_reference_value(&self, base: &Thingy, key: &Thingy) -> Result<Thingy, DeferError> {
        let st = base.static_type(self.0).defer()?.escape_of_non_nullable().vector_element_type(self.0)?.unwrap();
        Ok(VectorElementReferenceValue::new(&self.0.arena, base, key, &st).into())
    }

    pub fn create_static_reference_value(&self, base: &Thingy, property: &Thingy) -> Result<Thingy, DeferError> {
        Ok(StaticReferenceValue::new(&self.0.arena, base, property, &property.property_static_type(self.0).defer()?).into())
    }

    pub fn create_static_dynamic_reference_value(&self, base: &Thingy, qualifier: Option<Thingy>, key: &Thingy) -> Thingy {
        StaticDynamicReferenceValue::new(&self.0.arena, base, qualifier, key, &self.0.any_type()).into()
    }

    pub fn create_instance_reference_value(&self, base: &Thingy, property: &Thingy) -> Result<Thingy, DeferError> {
        Ok(InstanceReferenceValue::new(&self.0.arena, base, property, &property.property_static_type(self.0).defer()?).into())
    }

    pub fn create_tuple_reference_value(&self, base: &Thingy, index: usize) -> Thingy {
        let st = base.static_type(self.0).element_types().get(index).unwrap();
        TupleReferenceValue::new(&self.0.arena, base, index, &st).into()
    }

    pub fn create_scope_reference_value(&self, base: &Thingy, property: &Thingy) -> Result<Thingy, DeferError> {
        Ok(ScopeReferenceValue::new(&self.0.arena, base, property, &property.property_static_type(self.0).defer()?).into())
    }

    pub fn create_dynamic_scope_reference_value(&self, base: &Thingy, qualifier: Option<Thingy>, key: &Thingy) -> Thingy {
        DynamicScopeReferenceValue::new(&self.0.arena, base, qualifier, key, &self.0.any_type()).into()
    }

    pub fn create_package_reference_value(&self, base: &Thingy, property: &Thingy) -> Result<Thingy, DeferError> {
        Ok(PackageReferenceValue::new(&self.0.arena, base, property, &property.property_static_type(self.0).defer()?).into())
    }

    pub fn create_conversion_value(&self, base: &Thingy, variant: TypeConversionVariant, opt: bool, target: &Thingy) -> Result<Thingy, DeferError> {
        let mut st = if opt && !target.includes_null(self.0)? {
            if target.is::<NonNullableType>() {
                target.base()
            } else {
                self.create_nullable_type(target)
            }
        } else { target.clone() };
        if opt && !st.includes_null(self.0)? {
            st = self.create_nullable_type(target);
        }
        Ok(ConversionValue::new(&self.0.arena, base, variant, opt, target, &st).into())
    }

    pub fn create_lambda_object(&self, activation: &Thingy) -> Result<Thingy, DeferError> {
        Ok(LambdaObject::new(&self.0.arena, activation, &self.0.function_type().defer()?).into())
    }

    pub fn create_field_resolution(&self) -> Thingy {
        FieldResolution::new(&self.0.arena).into()
    }

    pub fn create_declarative_field_destructuring_resolution(&self) -> Thingy {
        DeclarativeFieldDestructuringResolution::new(&self.0.arena).into()
    }

    pub fn create_assignment_field_destructuring_resolution(&self) -> Thingy {
        AssignmentFieldDestructuringResolution::new(&self.0.arena).into()
    }
}