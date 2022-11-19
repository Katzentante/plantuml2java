
#[derive(Debug)]
pub struct Class<'a> {
    pub(crate) name: &'a str,
    attributes: Vec<Attribute<'a>>,
    methods: Vec<Function<'a>>,
    view: View,
    is_abstract: bool,
    inherits: Option<Box<Class<'a>>>,
}

impl<'a> Class<'a> {
    pub fn new(
        name: &'a str,
        attributes: Vec<Attribute<'a>>,
        methods: Vec<Function<'a>>,
        view: View,
        is_abstract: bool,
        inherits: Option<Box<Class<'a>>>,
    ) -> Self {
        Self {
            name,
            attributes,
            methods,
            view,
            is_abstract,
            inherits,
        }
    }

    pub fn build(name: &'a str, view: View, is_abstract: bool) -> Self {
        Self::new(name, Vec::new(), Vec::new(), view, is_abstract, None)
    }

    pub fn with_attribute(mut self, attribute: Attribute<'a>) -> Self {
        self.attributes.push(attribute);
        self
    }

    pub fn with_method(mut self, method: Function<'a>) -> Self {
        self.methods.push(method);
        self
    }

    pub fn inherits(mut self, class: Class<'a>) -> Self {
        self.inherits = Some(Box::new(class));
        self
    }

    pub fn as_string(&self) -> String {
        let mut str = String::new();

        // name itself
        str.push_str(self.view.as_string());
        str.push(' ');
        if self.is_abstract {
            str.push_str("abstract ");
        }
        str.push_str("class ");
        str.push_str(self.name);
        str.push(' ');
        str.push('{');
        str.push_str("\n");

        // attrbutes
        for p in self.attributes.iter() {
            str.push_str("    ");
            str.push_str(&p.as_string_as_attribute());
            str.push_str("\n");
        }

        // constructor
        str.push_str("\n");
        str.push_str("    ");
        str.push_str(&self.get_constructor_func().as_string());
        str.pop();
        str.push_str("\n");
        for p in self.attributes.iter() {
            str.push_str("        ");
            str.push_str(&format!("this.{} = {}", p.name, p.name));
            str.push_str("\n");
        }
        str.push_str("    ");
        str.push('}');
        str.push_str("\n\n");

        // methods
        for f in self.methods.iter() {
            str.push_str("    ");
            str.push_str(&f.as_string());
            str.push_str("\n");
        }

        str.push('}');
        str.push('\n');

        str
    }

    pub fn get_constructor_func(&self) -> Function<'a> {
        Function::new(
            self.name,
            self.view.clone(),
            Type::Other(""),
            self.attributes.clone(),
            false,
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Attribute<'a> {
    view: View,
    name: &'a str,
    vartype: Type<'a>,
    is_final: bool,
}

impl<'a> Attribute<'a> {
    pub fn new(view: View, name: &'a str, vartype: Type<'a>, is_final: bool) -> Self {
        Self {
            view,
            name,
            vartype,
            is_final,
        }
    }

    fn as_string_as_parameter(&self) -> String {
        let mut str = String::new();
        if self.is_final {
            str.push_str("final ");
        }
        str.push_str(self.vartype.as_string());
        str.push(' ');
        str.push_str(self.name);
        str.push_str(", ");
        str
    }

    fn as_string_as_attribute(&self) -> String {
        let mut str = String::new();
        if self.is_final {
            str.push_str("final ");
        }
        str.push_str(self.view.as_string());
        str.push(' ');
        str.push_str(self.vartype.as_string());
        str.push(' ');
        str.push_str(self.name);
        str.push(';');

        str
    }
}

#[derive(Debug)]
pub struct Function<'a> {
    name: &'a str,
    view: View,
    returntype: Type<'a>,
    parameters: Vec<Attribute<'a>>,
    is_abstract: bool,
}

impl<'a> Function<'a> {
    pub fn new(
        name: &'a str,
        view: View,
        returntype: Type<'a>,
        paremeters: Vec<Attribute<'a>>,
        is_abstract: bool,
    ) -> Self {
        Self {
            name,
            view,
            returntype,
            parameters: paremeters,
            is_abstract,
        }
    }

    pub fn as_string(&self) -> String {
        let mut str = String::new();
        if self.is_abstract {
            str.push_str("abstract");
            str.push(' ');
        }
        str.push_str(self.view.as_string());
        str.push(' ');
        str.push_str(self.returntype.as_string());
        str.push(' ');
        str.push_str(self.name);
        str.push('(');
        for p in self.parameters.iter() {
            str.push_str(&p.as_string_as_parameter());
        }
        if self.parameters.len() > 0 {
            str.pop();
            str.pop();
        }
        str.push_str(") {}");

        str
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Type<'a> {
    Other(&'a str),
}

impl<'a> Type<'a> {
    pub fn as_string(&self) -> &'a str {
        match self {
            Self::Other(s) => s,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum View {
    Normal,
    Private,
    Protected,
    Public,
}

impl View {
    pub fn as_string<'a>(&self) -> &'a str {
        match self {
            Self::Normal => "",
            Self::Public => "public",
            Self::Protected => "protected",
            Self::Private => "private",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
