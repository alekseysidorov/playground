# Пишем API с помощью процедурных макросов

Процедурные макросы это очень мощный инструмент кодогенерации, позволяющий обходиться без написания тонны
шаблонного кода или выражать какие-то новые концепции, как сделали, к примеру, разработчики крейта async_trait.
Но многие вполне обоснованно побаиваются пользоваться данным инструментом, в основном по причине того, что
разбор синтаксического дерева и атрибутов макроса превращается в "закатывание солнца вручную".
В данной статье я хочу поделиться некоторыми удачными, на мой взгляд, подходами к написанию процедурных макросов.

We need to go deeper

## Предисловие

Прежде всего нужно определиться с задачей, которую мы хотим решить с помощью макросов: мне часто очень хочется определить некоторое RPC API в виде трейта, который потом реализует как серверная часть, так и клиентская, а вот писать тонны шаблонного кода наоборот, совсем не хочется.

Сам API у нас будет выполнен по очень простому принципу: есть 4 типа запросов:

- GET запросы без параметров, с ними никаких проблем, пример: `/ping`.
- GET запросы с параметрами, параметры будут передаваться в виде URL query, пример: `/status?name=foo&count=15`.
- POST запросы без параметров.
- POST запросы с параметрами, которые передаются в виде json объектов.

Во всех случаях сервер будет отвечать валидным json объектом.

В идеале хочется определить нечто вот такое:

```rust
// Пишем определение интерфейса

#[derive(Debug, FromUrlQuery, Deserialize, Serialize)]
struct Query {
    first: String,
    second: u64,
}

/// Сам интерфейс, для которого макросом будет автоматически реализовываться
/// монтирование API поверх warp'а.
#[http_api(warp = "serve_ping_interface")]
trait PingInterface {
    #[http_api_endpoint(method = "get")]
    fn get(&self) -> Result<Query, Error>;
    #[http_api_endpoint(method = "get")]
    fn check(&self, query: Query) -> Result<bool, Error>;
    #[http_api_endpoint(method = "post")]
    fn set_value(&self, param: Query) -> Result<(), Error>;
    #[http_api_endpoint(method = "post")]
    fn increment(&self) -> Result<(), Error>;
}

// А теперь определяем обработчик запросов

#[derive(Debug, Default)]
struct ServiceInner {
    first: String,
    second: u64,
}

#[derive(Clone, Default)]
struct ServiceImpl(Arc<RwLock<ServiceInner>>);

impl ServiceImpl {
    fn new() -> Self {
        Self::default()
    }

    fn read(&self) -> RwLockReadGuard<ServiceInner> {
        self.0.read().unwrap()
    }

    fn write(&self) -> RwLockWriteGuard<ServiceInner> {
        self.0.write().unwrap()
    }
}

// Реализуем интерфейс для обработчика

impl PingInterface for ServiceImpl {
    fn get(&self) -> Result<Query, Error> {
        let inner = self.read();
        Ok(Query {
            first: inner.first.clone(),
            second: inner.second,
        })
    }

    fn check(&self, query: Query) -> Result<bool, Error> {
        let inner = self.read();
        Ok(inner.first == query.first && inner.second == query.second)
    }

    fn set_value(&self, param: Query) -> Result<(), Error> {
        let mut inner = self.write();
        inner.first = param.first;
        inner.second = param.second;
        Ok(())
    }

    fn increment(&self) -> Result<(), Error> {
        self.write().second += 1;
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    // А теперь просто вызываем сгенерированный код и наслаждаемся работающим API
    serve_ping_interface(ServiceImpl::new(), addr).await
}
```

Для начала напомню, что процедурными макросами в Rust'е называются специальные плагины к компилятору, которые получают на вход некоторое синтаксическое дерево с которым производят некоторые манипуляции и возращают новое.
В рамках этой статьи мы будем рассматривать два вида таких макросов: derive-макросы, которые многим уже знакомы по `serde`, и атрибутные макросы, которые можно использовать с куда более разнообразным набором токенов.

Так как крейты с процедурными макросами и являются по сути отдельной разновидностью, то мы разделим нашу функциональность на два крейта `http_api`, в котором будут содержаться необходимые трейты и вспомогательные методы, и `http_api_derive` с процедурными макросами.

## Делаем макрос FromUrlQuery

Как мне кажется, самый идеоматичный подход к написанию процедурных макросов - это объявление некоторого трейта, а потом создание макроса, который будет его выводить для разнообразных типов данных. Именно этот подход точно следует принципу наименьшего удивления, хотя никто не мешает в дерайве генерировать абсолютно любой код.

Итак, для начала объявим наш типаж по разбору URL query:

```rust
pub trait FromUrlQuery: Sized {
    fn from_query_str(query: &str) -> Result<Self, ParseQueryError>;
}
```

Далее давайте вместе напишем макрос, который будет генерировать код разбора запрос из URL'а в структуру `FooQuery`.
Данный макрос относится к типу макросов derive макросов и должен объявлятся следующим образом:

```rust
/// В данном случае макрос будет вызываться через `#[derive(FromUrlQuery)]`, а так-же будет иметь доступ
/// к атрибутам типа #[from_url_query(rename = "bar", skip, etc)]
#[proc_macro_derive(FromUrlQuery, attributes(from_url_query))]
pub fn from_url_query(input: TokenStream) -> TokenStream {
    from_url_query::impl_from_url_query(input)
}
```

Обычный джентельменский набор писателя процедурных макросов это `syn` и `quote`. Первый крейт является парсером `Rust` синтаксиса и содержит в себе все типы токенов, встречающиеся в синтаксическом дереве.
Во втором крейте содержится очень важный макрос `quote!`, который по сути является шаблонизатором и позволяет просто писать некоторый шаблонный Rust код, который будет потом преобразован в выходной набор токенов.
Помимо этих двух обязательных зависимостей есть еще много вспомогательных крейтов, одним из самых полезных, на мой взгяд, является `darling`. Этот крейт содержит набор процедурных макросов и типажей для облегчения типовых задач по разбору структур или атрибутов макросов.

## Spoiler

Обычно код разбора AST и атрибутов макроса превращается в такой вот кошмар, но в в случае использования
darling'а код будет короче и проще в поддержке.

```rust
fn get_field_names(input: &DeriveInput) -> Option<Vec<(Ident, Action)>> {
    let data = match &input.data {
        Data::Struct(x) => Some(x),
        Data::Enum(..) => None,
        _ => panic!("Protobuf convert can be derived for structs and enums only."),
    };
    data.map(|data| {
        data.fields
            .iter()
            .map(|f| {
                let mut action = Action::Convert;
                for attr in &f.attrs {
                    match attr.parse_meta() {
                        Ok(syn::Meta::List(ref meta)) if meta.ident == "protobuf_convert" => {
                            for nested in &meta.nested {
                                match nested {
                                    syn::NestedMeta::Meta(syn::Meta::Word(ident))
                                        if ident == "skip" =>
                                    {
                                        action = Action::Skip;
                                    }
                                    _ => {
                                        panic!("Unknown attribute");
                                    }
                                }
                            }
                        }
                        _ => {
                            // Other attributes are ignored
                        }
                    }
                }
                (f.ident.clone().unwrap(), action)
            })
            .collect()
    })
}

fn get_field_names_enum(input: &DeriveInput) -> Option<Vec<Ident>> {
    let data = match &input.data {
        Data::Struct(..) => None,
        Data::Enum(x) => Some(x),
        _ => panic!("Protobuf convert can be derived for structs and enums only."),
    };
    data.map(|data| data.variants.iter().map(|f| f.ident.clone()).collect())
}

fn implement_protobuf_convert_from_pb(field_names: &[(Ident, Action)]) -> impl quote::ToTokens {
    let mut to_convert = vec![];
    let mut to_skip = vec![];
    for (x, a) in field_names {
        match a {
            Action::Convert => to_convert.push(x),
            Action::Skip => to_skip.push(x),
        }
    }

    let getters = to_convert
        .iter()
        .map(|i| Ident::new(&format!("get_{}", i), Span::call_site()));
    let our_struct_names = to_convert.clone();
    let our_struct_names_skip = to_skip;

    quote! {
        fn from_pb(pb: Self::ProtoStruct) -> std::result::Result<Self, _FailureError> {
          Ok(Self {
           #( #our_struct_names: ProtobufConvert::from_pb(pb.#getters().to_owned())?, )*
           #( #our_struct_names_skip: Default::default(), )*
          })
        }
    }
}
```

Искомая структура, для которой мы выводим `FromUrlQuery`, не должна быть пустой, она не должна быть кортежем и для простоты эксперимента она не будет содержать шаблонных параметров и лайфтаймов. То есть, это всегда что-то такого вида:

```rust
#[derive(FromUrlQuery)]
struct OptionalQuery {
    first: String,
    opt_value: Option<u64>,
}
```

В `darling`'е есть несколько вспомогательных макросов.

Один из них - это `FromField`, который выводит десериализацию интересующих нас свойств поля структуры из синтаксического дерева:

```rust
#[derive(Clone, Debug, FromField)]
struct QueryField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
}
```

Хочу отметить, что при этом если бы мы хотели знать, публичное это поле или нет, то могли бы изменить сигнатуру на следующую:

```rust
#[derive(Clone, Debug, FromField)]
struct QueryField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    vis: syn::Visibility,
}
```

Второй интересующий нас макрос - это `FromDeriveInput`, который выводит десериализацию уже для целой структуры или перечисления:

```rust
#[derive(Debug, FromDeriveInput)]
// При помощи этого атрибута мы ограничиваемся поддежкой только именованых структур,
// если мы попробуем использовать наш макрос на других типах структур или перечислениях, то получим ошибку.
#[darling(supports(struct_named))]
struct FromUrlQuery {
    ident: syn::Ident,
    // В таком вот незамысловатом виде мы получаем список полей в уже разобранном виде.
    // В darling::ast::Data два шаблонных параметра: первый это поля перечисления, а второй это поля структуры.
    // Так как в данный момент перечисления нас не интересуют, то мы можем просто указать ().
    data: darling::ast::Data<(), QueryField>,
}
```

И все, на этом наш парсер готов.

```rust
let input: DeriveInput = syn::parse(input).unwrap();
let from_url_query = match FromUrlQuery::from_derive_input(&input) {
    Ok(parsed) => parsed,
    Err(e) => return e.write_errors().into(),
};
```

Можно приступать к написанию кодогенератора. Чтобы не перегружать статью сверх меры, мы просто будем делегировать десериализацию URL query в `serde`. Но при этом спрячем `serde` настолько глубоко, чтобы он не просочился в обязательные зависимости. Мы будем создавать точную копию нашей структуры и выводить для нее `Deserialize`, а для реального парсинга запросов будем использовать крейт `serde_urlencoded`. Но чтобы пользователям не приходилось самим добавлять serde в зависимости, мы в основном крейте сделаем реэкспорты.

```rust
#[doc(hidden)]
pub mod export {
    pub use serde;
    pub use serde_derive;
    pub use serde_urlencoded;
}
```

А теперь посмотрим, как же будет в реальности выглядеть процесс кодогенерации `FromUrlQuery`:

```rust
impl FromUrlQuery {
    // Чтобы не было конфликта имен, мы будем объявлять локальную структуру с префиксом Serde
    fn serde_wrapper_ident(&self) -> syn::Ident {
        let ident_str = format!("{}Serde", self.ident);
        syn::Ident::new(&ident_str, proc_macro2::Span::call_site())
    }

    /// Именно этот метод будет генерировать копию исходной структуры
    fn impl_serde_wrapper(&self) -> impl ToTokens {
        // Мы на уровне самого атрибута ограничили себя структурами с именоваными полями, поэтому
        // в данном случае unwrap совершенно безопасен.
        let fields = self.data.clone().take_struct().unwrap();
        // Тут мы для каждого из полей генерируем код присваивания как в случае
        // преобразования из Query в SerdeQuery, так и наоборот.
        let wrapped_fields = fields.iter().map(|field| {
            let ident = &field.ident;
            let ty = &field.ty;
            quote! { #ident: #ty }
        });
        let from_fields = fields.iter().map(|field| {
            let ident = &field.ident;
            quote! { #ident: v.#ident }
        });

        let wrapped_ident = self.serde_wrapper_ident();
        let ident = &self.ident;

        // В общем и целом: написание генерации кода с использованием quote! очень похоже на написание
        // правых частей в декларативных макросах за исключением того, что тут используется # вместо $.
        quote! {
            // Используем serde из экспортов нашего крейта.
            use http_api::export::serde_derive::Deserialize;

            #[derive(Deserialize)]
            // А чтобы сам serde использовал этот де реэкспорт, в нем есть прекрасный атрибут.
            #[serde(crate = "http_api::export::serde")]
            struct #wrapped_ident {
                #( #wrapped_fields, )*
            }

            impl From<#wrapped_ident> for #ident {
                fn from(v: #wrapped_ident) -> Self {
                    Self {
                        #( #from_fields, )*
                    }
                }
            }
        }
    }
}
```

Да, это все уже не выглядит слишком сложным или очень рутинным, по сути мы просто пишем то, что хотим получить, с одной стороны не сталкиваясь с большими когнитивными трудностями, а с другой стороны код получается весьма лаконичным и понятным. Но, к сожалению, не всегда жизнь бывает такой простой, все становится гораздо ~~интереснее~~ хуже, если есть необходимость написать не дерайв макрос, а атрибутный.

## Пишем макрос http_api

Вот тут нам уже не поможет `FromDeriveInput`, нет в darling'е готового набора инструментов для разбора типажей, так что придется нам закатывать солнце вручную. Но не так черт страшен, как его малюют, поэтому давайте приступим:

Код объявления атрибутных макросов несколько отличается и выглядит вот так:

```rust
#[proc_macro_attribute]
pub fn http_api(attr: TokenStream, item: TokenStream) -> TokenStream {
    // У нас уже два аргумента: отдельно синтаксическое дерево элемента, на который навешивается макрос
    // и отдельно ast самого атрибута.
    http_api::impl_http_api(attr, item)
}
```

Есть еще один крайне важный ньанс, над которым мне в свое время пришлось поломать голову: если мы просто решим, что раз уж нам дали целиком синтаксическое дерево трейта со всеми внутренними атрибутами типа `http_api_endpoint`, то мы будем лишь частично правы. Да, они будут видны при разборе `TokenStream`, но при этом же компилятор будет нам выдавать ошибку "cannot find attribute `http_api_endpoint` in this scope", что несколько сбивает с толку.
Чтобы решить эту проблему, нужно объявить пустой абтрибутный макрос `http_api_endpoint`, который просто будет возращать исходное синтаксическое дерево.

```rust
#[proc_macro_attribute]
#[doc(hidden)]
pub fn http_api_endpoint(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Мы не изменяем входной поток токенов, потому что `http_api_endpoint` лишь
    // предоставляет доступ к метаданным для `http_api` атрибута.

    // Однако все равно `http_api_endpoint` должен являться полноценным атрибутным макросом,
    // потому, что rustc не понимает неизвестных ему атрибутов.
    item
}
```

## Разбираем методы интерфейсного трейта

Для начала напишем код, который будет разбирать отдельный метод типажа с интерфейсом, который в общем случае будет выглядеть примерно так:

```rust
// Вариант для простых запросов:
#[http_api_endpoint(method = "#method_type")]
fn #method_name(&self) -> Result<$ResponseType, Error>;
// Вариант для запросов с параметрами:
#[http_api_endpoint(method = "#method_type")]
fn #method_name(&self, query: $QueryType) -> Result<$ResponseType, Error>;
```

Объявим типы HTTP запросов, которые мы умеем обрабатывать:

```rust
#[derive(Debug)]
enum SupportedHttpMethod {
    Get,
    Post,
}

impl FromMeta for SupportedHttpMethod {
    fn from_string(value: &str) -> Result<Self, darling::Error> {
        match value {
            "get" => Ok(SupportedHttpMethod::Get),
            "post" => Ok(SupportedHttpMethod::Post),
            other => Err(darling::Error::unknown_value(other)),
        }
    }
}
```

И объявим набор атрибутов для метода, которые мы можем указывать:

```rust
#[derive(Debug, FromMeta)]
struct EndpointAttrs {
    // Данный атрибут является обязательным, мы в любом случае должны указать тип запроса.
    method: SupportedHttpMethod,
    // Этот же атрибут является вспомогательным и по умолчанию будет принимать значение None,
    // если пользователь не напишет что-то типа такого:
    // #[http_api_endpoint(method = "get", rename = "foo")]
    #[darling(default)]
    rename: Option<String>,
}
```

Парсить мы будем сигнатуру функции, которая имеет тип `syn::Signature`, и в этом случае полностью
положится на помощь darling'а мы уже не сможем: большую часть разбора синтаксического дерева придется
писать самим, но вот атрибуты методов легко можно получить с помощью знакомого нам уже `FromMeta`.
А чтобы среди атрибутов метода отыскать нужный нам `http_api_endpoint` мы напишем небольшую
вспомогательную функцию. Мы будем преобразовывать тип нашего атрибута в `syn::NestedMeta` для того,
чтобы была возможность обрабатывать вложенные метаданные вида `(foo = "bar", boo(first, second))`.

```rust
fn find_meta_attrs(name: &str, args: &[syn::Attribute]) -> Option<syn::NestedMeta> {
    args.as_ref()
        .iter()
        .filter_map(|a| a.parse_meta().ok())
        .find(|m| m.path().is_ident(name))
        .map(syn::NestedMeta::from)
}
```

Теперь можно переходить к разбору сигнатуры, как я уже упоминал выше: нам нужно рассмотреть
два варианта: с дополнительным аргументом и без:

```rust
/// Вспомогательный метод для удобного создания ошибок.
fn invalid_method(span: &impl syn::spanned::Spanned) -> darling::Error {
    darling::Error::custom(
        "API method should have one of `fn foo(&self) -> Result<Bar, Error>` or \
         `fn foo(&self, arg: Foo) -> Result<Bar, Error>` form",
    )
    .with_span(span)
}

impl ParsedEndpoint {
    fn parse(sig: &syn::Signature, attrs: &[syn::Attribute]) -> Result<Self, darling::Error> {
        /// Создаем итератор с перечислением агрументов метода.
        let mut args = sig.inputs.iter();

        // Проверяем, что первый аргумент метода - это всегда &self и только он,
        // никакие варианты в &mut self, или с &self: Arc<Self> мы поддерживать не будем.
        if let Some(arg) = args.next() {
            match arg {
                // self в syn обозначается как Receiver.
                syn::FnArg::Receiver(syn::Receiver {
                    // Наличие reference говорит нам, что тип на самом деле &self.
                    reference: Some(_),
                    // А отсутствие mutability говорит о том, что в типе
                    // не содержится никаких mut.
                    mutability: None,
                    ..
                }) => {}
                _ => {
                    return Err(invalid_method(&arg));
                }
            }
        } else {
            return Err(invalid_method(&sig));
        }

        // Извлекаем опциональный тип параметра.
        let arg = args
            .next()
            .map(|arg| match arg {
                // FnArg может быть или Typed или Receiver, но receiver мы уже проверили
                // на предыдущем шаге, поэтому достаточно просто извлечь тип аргумента и все.
                syn::FnArg::Typed(arg) => Ok(arg.ty.clone()),
                // Только первый аргумент может быть self'ом.
                _ => unreachable!("Only first argument can be receiver."),
            })
            // Transpose очень удобная штука, которая превращает Option<Result<...>> в
            // Result<Option<...>>, чем очень улучшает читабельность кода.
            .transpose()?;

        // Извлекаем тип возращаемого значения, он тоже должен быть Typed, а не Receiver.
        let ret = match &sig.output {
            syn::ReturnType::Type(_, ty) => Ok(ty.clone()),
            _ => Err(invalid_method(&sig)),
        }?;

        // Функцией, которую мы писали выше, мы находим атрибуты относящиеся к нашему макросу.
        // И с помощью FromMeta::from_nested_meta извлекаем метаданные.
        let attrs = find_meta_attrs("http_api_endpoint", attrs)
            .map(|meta| EndpointAttrs::from_nested_meta(&&meta))
            .unwrap_or_else(|| Err(darling::Error::custom("todo")))?;

        /// Все, парсинг сигнатуры метода готов и можно возращать значение.
        Ok(Self {
            ident: sig.ident.clone(),
            arg,
            ret,
            attrs,
        })
    }
}
```

## Разбираем интерфейсный трейт целиком

Теперь можно приступить к разбору трейта с интерфейсом в целом. Типаж интерфейса всегда состоит
исключительно из методов, разбор которых мы описали выше, и каких-то дополнительных атрибутов.
Таким образом особых сложностей с его разбором не возникает.

```rust

/// Структура содержит исходный трейт, набор его атрибутов и список интерфейсных методов.
#[derive(Debug)]
struct ParsedApiDefinition {
    /// Исходный трейт, который мы не трогаем и который дальше как есть передается в
    /// исходящий поток токенов.
    item_trait: syn::ItemTrait,
    /// Список интерфейсных методов.
    endpoints: Vec<ParsedEndpoint>,
    /// Атрибуты интерфейсного типажа.
    attrs: ApiAttrs,
}

#[derive(Debug, FromMeta)]
struct ApiAttrs {
    /// Данный атрибут определяет имя функции, которая будет монтировать реализацию интерфейса
    /// к warp'у.
    warp: syn::Ident,
}

impl ParsedApiDefinition {
    fn parse(
        item_trait: syn::ItemTrait,
        attrs: &[syn::NestedMeta],
    ) -> Result<Self, darling::Error> {
        // С парсингом в данном случае все тривиально, среди итемов трейта мы фильтруем методы,
        // которые разбираем с помощью кода, который мы написали выше.
        let endpoints = item_trait
            .items
            .iter()
            .filter_map(|item| {
                if let syn::TraitItem::Method(method) = item {
                    Some(method)
                } else {
                    None
                }
            })
            .map(|method| ParsedEndpoint::parse(&method.sig, method.attrs.as_ref()))
            .collect::<Result<Vec<_>, darling::Error>>()?;

        // Парсим атрибуты трейта.
        let attrs = ApiAttrs::from_list(attrs)?;

        // И возращаем полностью разобранное описание HTTP API.
        Ok(Self {
            item_trait,
            endpoints,
            attrs,
        })
    }
}
```

## Пишем удобную обертку warp фильтрами

Чтобы написать кодогенерацию необходимо для начала разобраться с тем, как устроен warp и
каким образом к нему подключаются обработчики запросов. Все в warp'е крутится вокруг концепции,
которая называется `Filter`, фильтры можно комбинировать в цепочки при помощи комбинаторов `and`,
`map`, `and_then`, где каждый наложенный фильтр конкретизирует то, как будет обрабатываться запрос.
Например, если мы хотим просто написать обработчик запросов, который на GET запрос будет просто
возращать некоторый json, то мы просто пишем что-то в таком стиле:

```rust
/// Мы оборачиваем некоторый обработчик, который просто отдает результат
/// в фильтр warp'а
pub fn simple_get<F, R, E>(name: &'static str, handler: F) -> JsonReply
where
    F: Fn() -> Result<R, E> + Clone + Send + Sync + 'static,
    R: ser::Serialize,
    E: Reject,
{
    // Создаем некоторый фильтр запросов, который принимает только GET запросы,
    // а остальные игнорирует.
    warp::get()
        // Накладываем на него дополнительную фильтрацию, чтобы он принимал
        // только запросы с путем {name}
        .and(warp::path(name))
        // А в последнем комбинаторе and_then мы вызываем обработчик и получившийся
        // результат возращаем в виде json объекта
        .and_then(move || {
            let handler = handler.clone();
            // Обработчик запросов в реальности всегда асинхронный, поэтому нам нужно
            // обернуть наш синхронный вызов в async блок.
            async move {
                match handler() {
                    Ok(value) => Ok(warp::reply::json(&value)),
                    // В warp'е достаточно своеобразная система обработки ошибок,
                    // но чтобы не перегружать статью, мы не будем касаться этого вопроса
                    // подробно и просто передадим какой-то тип ошибки наверх.
                    Err(e) => Err(warp::reject::custom(e)),
                }
            }
        })
        .boxed()
}
```

Для случая с GET запросами с параметрами мы лишь немного изменим обертку, которую мы написали выше,
добавив еще один фильтр в цепочку.

```rust
pub fn query_get<F, Q, R, E>(name: &'static str, handler: F) -> JsonReply
where
    F: Fn(Q) -> Result<R, E> + Clone + Send + Sync + 'static,
    Q: FromUrlQuery,
    R: ser::Serialize,
    E: Reject,
{
    warp::get()
        .and(warp::path(name))
        // Добавляем в цепочку фильтров фильтрацию по URL query, который вернет нам
        // строчку с необработанным query.
        .and(warp::filters::query::raw())
        .and_then(move |raw_query: String| {
            let handler = handler.clone();
            async move {
                // Применим к строчке с запросом написанный нами ранее трейт FromUrlQuery
                // и получим переменную с нужным обработчику запросов типом.
                let query = Q::from_query_str(&raw_query)
                    .map_err(|_| warp::reject::custom(IncorrectQuery))?;

                match handler(query) {
                    Ok(value) => Ok(warp::reply::json(&value)),
                    Err(e) => Err(warp::reject::custom(e)),
                }
            }
        })
        .boxed()
}
```

Обработчики остальных двух типов запросов пишутся точно так-же по аналогии.

## Собираем обработчики воедино

Помимо комбинаторов `and`, которое объединяет фильтры в цепочку, существует еще комбинатор
`or`, которое позволяет выбирать из двух фильтров подходящий по ситуации, фактически, таким
образом мы организовываем роутинг запросов, причем правила могут быть очень сложными.
Давайте просто взглянем на пример из документации:

```rust
use std::net::SocketAddr;
use warp::Filter;

// В данном примере warp будет обрабатывать запросы вида `/:u32` или
// `/:socketaddr`
warp::path::param::<u32>()
    .or(warp::path::param::<SocketAddr>());
```

И вот теперь мы можем приступить к генерации тела искомой функции
`serve_ping_interface`. Для начала реализуем генерацию соответствующих
warp фильтров для соответствующих методов трейта, где service это объект,
реализующий бизнес-логику.

```rust
impl ParsedEndpoint {
    fn impl_endpoint_handler(&self) -> impl ToTokens {
        // Имя метода является используется в качестве пути.
        let path = self.endpoint_path();
        let ident = &self.ident;

        // Перебираем все четыре варианта и с помощью оберток, описанных выше
        // создаем соответствующие warp фильтры.
        match (&self.attrs.method, &self.arg) {
            (SupportedHttpMethod::Get, None) => {
                quote! {
                    let #ident = http_api::warp_backend::simple_get(#path, {
                        let out = service.clone();
                        move || out.#ident()
                    });
                }
            }

            (SupportedHttpMethod::Get, Some(_arg)) => {
                quote! {
                    let #ident = http_api::warp_backend::query_get(#path, {
                        let out = service.clone();
                        move |query| out.#ident(query)
                    });
                }
            }

            (SupportedHttpMethod::Post, None) => {
                quote! {
                    let #ident = http_api::warp_backend::simple_post(#path, {
                        let out = service.clone();
                        move || out.#ident()
                    });
                }
            }

            (SupportedHttpMethod::Post, Some(_arg)) => {
                quote! {
                    let #ident = http_api::warp_backend::params_post(#path, {
                        let out = service.clone();
                        move |params| out.#ident(params)
                    });
                }
            }
        }
    }
}
```

А теперь с помощью комбинатора `or` собираем все фильтры воедино.

```rust
impl ToTokens for ParsedApiDefinition {
    fn to_tokens(&self, out: &mut proc_macro2::TokenStream) {
        let fn_name = &self.attrs.warp;
        let interface = &self.item_trait.ident;

        // В первом массиве содержатся фильтры, а во втором массиве
        // соответствющием им идентификаторы, в данном случае это
        // имена методов интерфейсного типажа.
        let (filters, idents): (Vec<_>, Vec<_>) = self
            .endpoints
            .iter()
            .map(|endpoint| {
                let ident = &endpoint.ident;
                let handler = endpoint.impl_endpoint_handler();

                (handler, ident)
            })
            .unzip();

        let mut tail = idents.into_iter();
        // Нам нужно собрать все фильтры в конструкцию вида
        // `a.or(b).or(c).or(d)`, а для этого головной элемент
        // следует обрабатывать отдельно, поэтому извлекаем его.
        let head = tail.next().unwrap();
        let serve_impl = quote! {
            #head #( .or(#tail) )*
        };

        // Финальный аккорд: создаем искомую функцию.
        let tokens = quote! {
            fn #fn_name<T>(
                service: T,
                addr: impl Into<std::net::SocketAddr>,
            ) -> impl std::future::Future<Output = ()>
            where
                T: #interface + Clone + Send + Sync + 'static,
            {
                use warp::Filter;

                // Создаем фильтры для всех методов трейта.
                #( #filters )*

                // Комбинируем все фильтры в конечное API и запускаем
                // warp сервер.
                warp::serve(#serve_impl).run(addr.into())
            }

        };
        out.extend(tokens)
    }
}
```

## Заключение

С помощью этой статьи я хотел показать, что derive макросы не всегда так сложны в написании,
если использовать дополнительные библиотеки и следовать определенным практикам.
На мой взгляд, подобный подход к использованию трейтов наиболее удобен, если нужно описать
некоторый RPC, связывающий различные приложения, которые написаны на Rust'е.
Нетрудно заметить, что можно легко написать генератор реализации типажа-интерфейса для http
клиентов типа reqwest и тем самым исключить возможность ошибиться в сопряжении клиента и сервера
на корню.

Никто не мешает при помощи макросов пойти дальше и выводить еще и openapi или swagger
спецификацию для типажей-интерфейсов. Но в таком случае мне кажется, что лучше пойти другим путем и
по спецификации написать генератор rust кода, это даст больший простор для маневров, а если писать этот
генератор в виде build зависимости, то никто не мешает воспользоваться мощью `syn` и `quote` и в этом случае.

Полностью рабочий код, примеры которого приводились в данной статье можно найти по этой ссылке, спасибо за внимание.
