// @generated
impl serde::Serialize for ListUserReq {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.user_id.is_some() {
            len += 1;
        }
        if self.page.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("admin.v1.ListUserReq", len)?;
        if let Some(v) = self.user_id.as_ref() {
            struct_ser.serialize_field("UserId", v)?;
        }
        if let Some(v) = self.page.as_ref() {
            struct_ser.serialize_field("Page", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUserReq {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UserId",
            "Page",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserId,
            Page,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "UserId" => Ok(GeneratedField::UserId),
                            "Page" => Ok(GeneratedField::Page),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListUserReq;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct admin.v1.ListUserReq")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUserReq, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_id__ = None;
                let mut page__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("UserId"));
                            }
                            user_id__ = map_.next_value()?;
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("Page"));
                            }
                            page__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListUserReq {
                    user_id: user_id__,
                    page: page__,
                })
            }
        }
        deserializer.deserialize_struct("admin.v1.ListUserReq", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUserResp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.response_metadata.is_some() {
            len += 1;
        }
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("admin.v1.ListUserResp", len)?;
        if let Some(v) = self.response_metadata.as_ref() {
            struct_ser.serialize_field("ResponseMetadata", v)?;
        }
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("Result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUserResp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ResponseMetadata",
            "Result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResponseMetadata,
            Result,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "ResponseMetadata" => Ok(GeneratedField::ResponseMetadata),
                            "Result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListUserResp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct admin.v1.ListUserResp")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUserResp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut response_metadata__ = None;
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResponseMetadata => {
                            if response_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ResponseMetadata"));
                            }
                            response_metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("Result"));
                            }
                            result__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListUserResp {
                    response_metadata: response_metadata__,
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("admin.v1.ListUserResp", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListUserResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.eow.is_empty() {
            len += 1;
        }
        if self.total != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("admin.v1.ListUserResult", len)?;
        if !self.eow.is_empty() {
            struct_ser.serialize_field("Eow", &self.eow)?;
        }
        if self.total != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("Total", ToString::to_string(&self.total).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListUserResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "Eow",
            "Total",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Eow,
            Total,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "Eow" => Ok(GeneratedField::Eow),
                            "Total" => Ok(GeneratedField::Total),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListUserResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct admin.v1.ListUserResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListUserResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut eow__ = None;
                let mut total__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Eow => {
                            if eow__.is_some() {
                                return Err(serde::de::Error::duplicate_field("Eow"));
                            }
                            eow__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Total => {
                            if total__.is_some() {
                                return Err(serde::de::Error::duplicate_field("Total"));
                            }
                            total__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ListUserResult {
                    eow: eow__.unwrap_or_default(),
                    total: total__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("admin.v1.ListUserResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoginReq {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.query_key.is_empty() {
            len += 1;
        }
        if !self.password.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("admin.v1.LoginReq", len)?;
        if !self.query_key.is_empty() {
            struct_ser.serialize_field("QueryKey", &self.query_key)?;
        }
        if !self.password.is_empty() {
            struct_ser.serialize_field("Password", &self.password)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoginReq {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "QueryKey",
            "Password",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            QueryKey,
            Password,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "QueryKey" => Ok(GeneratedField::QueryKey),
                            "Password" => Ok(GeneratedField::Password),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoginReq;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct admin.v1.LoginReq")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoginReq, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut query_key__ = None;
                let mut password__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::QueryKey => {
                            if query_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("QueryKey"));
                            }
                            query_key__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("Password"));
                            }
                            password__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LoginReq {
                    query_key: query_key__.unwrap_or_default(),
                    password: password__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("admin.v1.LoginReq", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoginResp {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.response_metadata.is_some() {
            len += 1;
        }
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("admin.v1.LoginResp", len)?;
        if let Some(v) = self.response_metadata.as_ref() {
            struct_ser.serialize_field("ResponseMetadata", v)?;
        }
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("Result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoginResp {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ResponseMetadata",
            "Result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResponseMetadata,
            Result,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "ResponseMetadata" => Ok(GeneratedField::ResponseMetadata),
                            "Result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoginResp;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct admin.v1.LoginResp")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoginResp, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut response_metadata__ = None;
                let mut result__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResponseMetadata => {
                            if response_metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ResponseMetadata"));
                            }
                            response_metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("Result"));
                            }
                            result__ = map_.next_value()?;
                        }
                    }
                }
                Ok(LoginResp {
                    response_metadata: response_metadata__,
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("admin.v1.LoginResp", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LoginResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("admin.v1.LoginResult", len)?;
        if !self.token.is_empty() {
            struct_ser.serialize_field("Token", &self.token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LoginResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "Token",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Token,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "Token" => Ok(GeneratedField::Token),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LoginResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct admin.v1.LoginResult")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LoginResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Token => {
                            if token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("Token"));
                            }
                            token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LoginResult {
                    token: token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("admin.v1.LoginResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for User {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.username.is_empty() {
            len += 1;
        }
        if !self.user_id.is_empty() {
            len += 1;
        }
        if self.age != 0 {
            len += 1;
        }
        if !self.email.is_empty() {
            len += 1;
        }
        if !self.phone.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("admin.v1.User", len)?;
        if !self.username.is_empty() {
            struct_ser.serialize_field("Username", &self.username)?;
        }
        if !self.user_id.is_empty() {
            struct_ser.serialize_field("UserId", &self.user_id)?;
        }
        if self.age != 0 {
            struct_ser.serialize_field("Age", &self.age)?;
        }
        if !self.email.is_empty() {
            struct_ser.serialize_field("Email", &self.email)?;
        }
        if !self.phone.is_empty() {
            struct_ser.serialize_field("Phone", &self.phone)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for User {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "Username",
            "UserId",
            "Age",
            "Email",
            "Phone",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Username,
            UserId,
            Age,
            Email,
            Phone,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "Username" => Ok(GeneratedField::Username),
                            "UserId" => Ok(GeneratedField::UserId),
                            "Age" => Ok(GeneratedField::Age),
                            "Email" => Ok(GeneratedField::Email),
                            "Phone" => Ok(GeneratedField::Phone),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = User;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct admin.v1.User")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<User, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut username__ = None;
                let mut user_id__ = None;
                let mut age__ = None;
                let mut email__ = None;
                let mut phone__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Username => {
                            if username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("Username"));
                            }
                            username__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserId => {
                            if user_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("UserId"));
                            }
                            user_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Age => {
                            if age__.is_some() {
                                return Err(serde::de::Error::duplicate_field("Age"));
                            }
                            age__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("Email"));
                            }
                            email__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Phone => {
                            if phone__.is_some() {
                                return Err(serde::de::Error::duplicate_field("Phone"));
                            }
                            phone__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(User {
                    username: username__.unwrap_or_default(),
                    user_id: user_id__.unwrap_or_default(),
                    age: age__.unwrap_or_default(),
                    email: email__.unwrap_or_default(),
                    phone: phone__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("admin.v1.User", FIELDS, GeneratedVisitor)
    }
}
