pub mod admin_gen {
    #![allow(warnings, clippy::all)]

    pub mod base {

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default)]
        #[serde(rename_all = "PascalCase")]
        #[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
        pub struct ErrorDto {
            pub code_n: i32,

            pub code: ::pilota::FastStr,

            pub message: ::pilota::FastStr,

            pub status_code: i32,
        }
        impl ::pilota::prost::Message for ErrorDto {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + ::pilota::prost::encoding::int32::encoded_len(1, &self.code_n)
                    + ::pilota::prost::encoding::faststr::encoded_len(2, &self.code)
                    + ::pilota::prost::encoding::faststr::encoded_len(3, &self.message)
                    + ::pilota::prost::encoding::int32::encoded_len(4, &self.status_code)
            }

            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::pilota::prost::bytes::BufMut,
            {
                ::pilota::prost::encoding::int32::encode(1, &self.code_n, buf);
                ::pilota::prost::encoding::faststr::encode(2, &self.code, buf);
                ::pilota::prost::encoding::faststr::encode(3, &self.message, buf);
                ::pilota::prost::encoding::int32::encode(4, &self.status_code, buf);
            }

            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::pilota::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::pilota::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::pilota::prost::DecodeError>
            where
                B: ::pilota::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = stringify!(ErrorDto);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.code_n;
                        ::pilota::prost::encoding::int32::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(code_n));
                            error
                        })
                    }
                    2 => {
                        let mut _inner_pilota_value = &mut self.code;
                        ::pilota::prost::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(code));
                            error
                        })
                    }
                    3 => {
                        let mut _inner_pilota_value = &mut self.message;
                        ::pilota::prost::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(message));
                            error
                        })
                    }
                    4 => {
                        let mut _inner_pilota_value = &mut self.status_code;
                        ::pilota::prost::encoding::int32::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(status_code));
                            error
                        })
                    }
                    _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default)]
        #[serde(rename_all = "PascalCase")]
        #[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
        pub struct ResponseMetadataDto {
            pub request_id: ::pilota::FastStr,

            pub error: ::std::option::Option<ErrorDto>,
        }
        impl ::pilota::prost::Message for ResponseMetadataDto {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + ::pilota::prost::encoding::faststr::encoded_len(1, &self.request_id)
                    + self.error.as_ref().map_or(0, |msg| {
                        ::pilota::prost::encoding::message::encoded_len(100, msg)
                    })
            }

            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::pilota::prost::bytes::BufMut,
            {
                ::pilota::prost::encoding::faststr::encode(1, &self.request_id, buf);
                if let Some(_pilota_inner_value) = self.error.as_ref() {
                    ::pilota::prost::encoding::message::encode(100, _pilota_inner_value, buf);
                }
            }

            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::pilota::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::pilota::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::pilota::prost::DecodeError>
            where
                B: ::pilota::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = stringify!(ResponseMetadataDto);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.request_id;
                        ::pilota::prost::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(request_id));
                            error
                        })
                    }
                    100 => {
                        let mut _inner_pilota_value = &mut self.error;
                        ::pilota::prost::encoding::message::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(error));
                            error
                        })
                    }
                    _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default)]
        #[serde(rename_all = "PascalCase")]
        #[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
        pub struct PageReqDto {
            pub page_num: i64,

            pub page_size: i64,
        }
        impl ::pilota::prost::Message for PageReqDto {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + ::pilota::prost::encoding::int64::encoded_len(1, &self.page_num)
                    + ::pilota::prost::encoding::int64::encoded_len(2, &self.page_size)
            }

            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::pilota::prost::bytes::BufMut,
            {
                ::pilota::prost::encoding::int64::encode(1, &self.page_num, buf);
                ::pilota::prost::encoding::int64::encode(2, &self.page_size, buf);
            }

            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::pilota::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::pilota::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::pilota::prost::DecodeError>
            where
                B: ::pilota::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = stringify!(PageReqDto);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.page_num;
                        ::pilota::prost::encoding::int64::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(page_num));
                            error
                        })
                    }
                    2 => {
                        let mut _inner_pilota_value = &mut self.page_size;
                        ::pilota::prost::encoding::int64::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(page_size));
                            error
                        })
                    }
                    _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
    }

    pub mod v1 {

        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default)]
        #[serde(rename_all = "PascalCase")]
        #[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
        pub struct LoginResultDto {
            pub token: ::pilota::FastStr,
        }
        impl ::pilota::prost::Message for LoginResultDto {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + ::pilota::prost::encoding::faststr::encoded_len(1, &self.token)
            }

            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::pilota::prost::bytes::BufMut,
            {
                ::pilota::prost::encoding::faststr::encode(1, &self.token, buf);
            }

            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::pilota::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::pilota::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::pilota::prost::DecodeError>
            where
                B: ::pilota::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = stringify!(LoginResultDto);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.token;
                        ::pilota::prost::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(token));
                            error
                        })
                    }
                    _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default)]
        #[serde(rename_all = "PascalCase")]
        #[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
        pub struct UserDto {
            pub username: ::pilota::FastStr,

            pub user_id: ::pilota::FastStr,

            pub age: i32,

            pub email: ::pilota::FastStr,

            pub phone: ::pilota::FastStr,
        }
        impl ::pilota::prost::Message for UserDto {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + ::pilota::prost::encoding::faststr::encoded_len(1, &self.username)
                    + ::pilota::prost::encoding::faststr::encoded_len(2, &self.user_id)
                    + ::pilota::prost::encoding::int32::encoded_len(3, &self.age)
                    + ::pilota::prost::encoding::faststr::encoded_len(4, &self.email)
                    + ::pilota::prost::encoding::faststr::encoded_len(5, &self.phone)
            }

            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::pilota::prost::bytes::BufMut,
            {
                ::pilota::prost::encoding::faststr::encode(1, &self.username, buf);
                ::pilota::prost::encoding::faststr::encode(2, &self.user_id, buf);
                ::pilota::prost::encoding::int32::encode(3, &self.age, buf);
                ::pilota::prost::encoding::faststr::encode(4, &self.email, buf);
                ::pilota::prost::encoding::faststr::encode(5, &self.phone, buf);
            }

            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::pilota::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::pilota::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::pilota::prost::DecodeError>
            where
                B: ::pilota::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = stringify!(UserDto);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.username;
                        ::pilota::prost::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(username));
                            error
                        })
                    }
                    2 => {
                        let mut _inner_pilota_value = &mut self.user_id;
                        ::pilota::prost::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(user_id));
                            error
                        })
                    }
                    3 => {
                        let mut _inner_pilota_value = &mut self.age;
                        ::pilota::prost::encoding::int32::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(age));
                            error
                        })
                    }
                    4 => {
                        let mut _inner_pilota_value = &mut self.email;
                        ::pilota::prost::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(email));
                            error
                        })
                    }
                    5 => {
                        let mut _inner_pilota_value = &mut self.phone;
                        ::pilota::prost::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(phone));
                            error
                        })
                    }
                    _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default)]
        #[serde(rename_all = "PascalCase")]
        #[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
        pub struct LoginRespDto {
            pub response_metadata: ::std::option::Option<super::base::ResponseMetadataDto>,

            pub result: ::std::option::Option<LoginResultDto>,
        }
        impl ::pilota::prost::Message for LoginRespDto {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + self.response_metadata.as_ref().map_or(0, |msg| {
                    ::pilota::prost::encoding::message::encoded_len(1, msg)
                }) + self.result.as_ref().map_or(0, |msg| {
                    ::pilota::prost::encoding::message::encoded_len(2, msg)
                })
            }

            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::pilota::prost::bytes::BufMut,
            {
                if let Some(_pilota_inner_value) = self.response_metadata.as_ref() {
                    ::pilota::prost::encoding::message::encode(1, _pilota_inner_value, buf);
                }
                if let Some(_pilota_inner_value) = self.result.as_ref() {
                    ::pilota::prost::encoding::message::encode(2, _pilota_inner_value, buf);
                }
            }

            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::pilota::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::pilota::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::pilota::prost::DecodeError>
            where
                B: ::pilota::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = stringify!(LoginRespDto);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.response_metadata;
                        ::pilota::prost::encoding::message::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(response_metadata));
                            error
                        })
                    }
                    2 => {
                        let mut _inner_pilota_value = &mut self.result;
                        ::pilota::prost::encoding::message::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(result));
                            error
                        })
                    }
                    _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default)]
        #[serde(rename_all = "PascalCase")]
        #[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
        pub struct ListUserResultDto {
            pub row: ::std::vec::Vec<UserDto>,

            pub total: i64,
        }
        impl ::pilota::prost::Message for ListUserResultDto {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + ::pilota::prost::encoding::message::encoded_len_repeated(1, &self.row)
                    + ::pilota::prost::encoding::int64::encoded_len(99, &self.total)
            }

            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::pilota::prost::bytes::BufMut,
            {
                for msg in &self.row {
                    ::pilota::prost::encoding::message::encode(1, msg, buf);
                }
                ::pilota::prost::encoding::int64::encode(99, &self.total, buf);
            }

            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::pilota::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::pilota::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::pilota::prost::DecodeError>
            where
                B: ::pilota::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = stringify!(ListUserResultDto);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.row;
                        ::pilota::prost::encoding::message::merge_repeated(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(row));
                            error
                        })
                    }
                    99 => {
                        let mut _inner_pilota_value = &mut self.total;
                        ::pilota::prost::encoding::int64::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(total));
                            error
                        })
                    }
                    _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default)]
        #[serde(rename_all = "PascalCase")]
        #[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
        pub struct ListUserReqDto {
            pub username: ::std::option::Option<::pilota::FastStr>,

            pub user_id: ::std::option::Option<::pilota::FastStr>,

            pub page: ::std::option::Option<super::base::PageReqDto>,
        }
        impl ::pilota::prost::Message for ListUserReqDto {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + self.username.as_ref().map_or(0, |value| {
                    ::pilota::prost::encoding::faststr::encoded_len(1, value)
                }) + self.user_id.as_ref().map_or(0, |value| {
                    ::pilota::prost::encoding::faststr::encoded_len(2, value)
                }) + self.page.as_ref().map_or(0, |msg| {
                    ::pilota::prost::encoding::message::encoded_len(3, msg)
                })
            }

            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::pilota::prost::bytes::BufMut,
            {
                if let Some(_pilota_inner_value) = self.username.as_ref() {
                    ::pilota::prost::encoding::faststr::encode(1, _pilota_inner_value, buf);
                };
                if let Some(_pilota_inner_value) = self.user_id.as_ref() {
                    ::pilota::prost::encoding::faststr::encode(2, _pilota_inner_value, buf);
                };
                if let Some(_pilota_inner_value) = self.page.as_ref() {
                    ::pilota::prost::encoding::message::encode(3, _pilota_inner_value, buf);
                }
            }

            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::pilota::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::pilota::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::pilota::prost::DecodeError>
            where
                B: ::pilota::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = stringify!(ListUserReqDto);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.username;
                        ::pilota::prost::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(username));
                            error
                        })
                    }
                    2 => {
                        let mut _inner_pilota_value = &mut self.user_id;
                        ::pilota::prost::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(user_id));
                            error
                        })
                    }
                    3 => {
                        let mut _inner_pilota_value = &mut self.page;
                        ::pilota::prost::encoding::message::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(page));
                            error
                        })
                    }
                    _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }

        pub trait AdminService {}
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default)]
        #[serde(rename_all = "PascalCase")]
        #[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
        pub struct ListUserRespDto {
            pub response_metadata: ::std::option::Option<super::base::ResponseMetadataDto>,

            pub result: ::std::option::Option<ListUserResultDto>,
        }
        impl ::pilota::prost::Message for ListUserRespDto {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + self.response_metadata.as_ref().map_or(0, |msg| {
                    ::pilota::prost::encoding::message::encoded_len(1, msg)
                }) + self.result.as_ref().map_or(0, |msg| {
                    ::pilota::prost::encoding::message::encoded_len(2, msg)
                })
            }

            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::pilota::prost::bytes::BufMut,
            {
                if let Some(_pilota_inner_value) = self.response_metadata.as_ref() {
                    ::pilota::prost::encoding::message::encode(1, _pilota_inner_value, buf);
                }
                if let Some(_pilota_inner_value) = self.result.as_ref() {
                    ::pilota::prost::encoding::message::encode(2, _pilota_inner_value, buf);
                }
            }

            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::pilota::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::pilota::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::pilota::prost::DecodeError>
            where
                B: ::pilota::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = stringify!(ListUserRespDto);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.response_metadata;
                        ::pilota::prost::encoding::message::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(response_metadata));
                            error
                        })
                    }
                    2 => {
                        let mut _inner_pilota_value = &mut self.result;
                        ::pilota::prost::encoding::message::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(result));
                            error
                        })
                    }
                    _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
        #[derive(PartialOrd, Hash, Eq, Ord, Debug, Default)]
        #[serde(rename_all = "PascalCase")]
        #[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
        pub struct LoginReqDto {
            pub username: ::std::option::Option<::pilota::FastStr>,

            pub user_id: ::std::option::Option<::pilota::FastStr>,

            pub password: ::pilota::FastStr,
        }
        impl ::pilota::prost::Message for LoginReqDto {
            #[inline]
            fn encoded_len(&self) -> usize {
                0 + self.username.as_ref().map_or(0, |value| {
                    ::pilota::prost::encoding::faststr::encoded_len(1, value)
                }) + self.user_id.as_ref().map_or(0, |value| {
                    ::pilota::prost::encoding::faststr::encoded_len(2, value)
                }) + ::pilota::prost::encoding::faststr::encoded_len(3, &self.password)
            }

            #[allow(unused_variables)]
            fn encode_raw<B>(&self, buf: &mut B)
            where
                B: ::pilota::prost::bytes::BufMut,
            {
                if let Some(_pilota_inner_value) = self.username.as_ref() {
                    ::pilota::prost::encoding::faststr::encode(1, _pilota_inner_value, buf);
                };
                if let Some(_pilota_inner_value) = self.user_id.as_ref() {
                    ::pilota::prost::encoding::faststr::encode(2, _pilota_inner_value, buf);
                };
                ::pilota::prost::encoding::faststr::encode(3, &self.password, buf);
            }

            #[allow(unused_variables)]
            fn merge_field<B>(
                &mut self,
                tag: u32,
                wire_type: ::pilota::prost::encoding::WireType,
                buf: &mut B,
                ctx: ::pilota::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::pilota::prost::DecodeError>
            where
                B: ::pilota::prost::bytes::Buf,
            {
                const STRUCT_NAME: &'static str = stringify!(LoginReqDto);

                match tag {
                    1 => {
                        let mut _inner_pilota_value = &mut self.username;
                        ::pilota::prost::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(username));
                            error
                        })
                    }
                    2 => {
                        let mut _inner_pilota_value = &mut self.user_id;
                        ::pilota::prost::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value
                                .get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(user_id));
                            error
                        })
                    }
                    3 => {
                        let mut _inner_pilota_value = &mut self.password;
                        ::pilota::prost::encoding::faststr::merge(
                            wire_type,
                            _inner_pilota_value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, stringify!(password));
                            error
                        })
                    }
                    _ => ::pilota::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
        }
    }
}
