#[derive(Debug, Clone)]
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

    pub fn with_abstract(mut self, is_abstract: bool) -> Self {
        self.is_abstract = is_abstract;
        self
    }

    pub fn to_java(&self) -> String {
        let mut str = String::new();

        // top name and class dfinition
        str.push_str(self.view.to_java());
        str.push(' ');
        if self.is_abstract {
            str.push_str("abstract ");
        }
        str.push_str("class ");
        str.push_str(self.name);
        str.push(' ');
        match &self.inherits {
            Some(c) => {
                str.push_str("extends ");
                str.push_str(c.name);
            }
            None => (),
        }
        str.push('{');
        str.push_str("\n");

        // attrbutes
        for p in self.attributes.iter() {
            str.push_str("    ");
            str.push_str(&p.to_java_as_attribute());
            str.push_str("\n");
        }

        // constructor func
        // top row
        str.push('\n');
        str.push_str("    ");
        str.push_str(&self.get_constructor_func().to_java());
        str.pop();
        str.push('\n');
        // super func if inherits
        match &self.inherits {
            Some(class) => {
                str.push_str("        super(");
                for attr in class.attributes.iter() {
                    str.push_str(&attr.name);
                    str.push(',');
                    str.push(' ');
                }
                if class.attributes.len() > 0 {
                    str.pop();
                    str.pop();
                }
                str.push(')');
                str.push(';');
            }
            None => (),
        }
        str.push('\n');
        // normal this. thisngs
        for p in self.attributes.iter() {
            str.push_str("        ");
            str.push_str(&format!("this.{} = {};", p.name, p.name));
            str.push_str("\n");
        }
        str.push_str("    ");
        str.push('}');
        str.push_str("\n\n");

        // methods
        for f in self.methods.iter() {
            str.push_str("    ");
            str.push_str(&f.to_java());
            str.push_str("\n");
        }

        str.push('}');
        str.push('\n');

        str
    }

    pub fn get_constructor_func(&self) -> Function<'a> {
        let mut attributes = self.attributes.clone();
        match &self.inherits {
            Some(c) => {
                for attr in c.attributes.iter() {
                    attributes.push(*attr);
                }
            }
            None => (),
        }
        Function::new(
            self.name,
            self.view.clone(),
            Type::Other(""),
            attributes,
            false,
            false,
        )
    }

    pub fn set_inherits(&mut self, master: Class<'a>) {
        self.inherits = Some(Box::new(master));
    }

    pub fn get_inherits(&self) -> Option<Box<Class>> {
        self.inherits.clone()
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

    fn to_java_as_parameter(&self) -> String {
        let mut str = String::new();
        if self.is_final {
            str.push_str("final ");
        }
        str.push_str(self.vartype.to_java());
        str.push(' ');
        str.push_str(self.name);
        str.push_str(", ");
        str
    }

    fn to_java_as_attribute(&self) -> String {
        let mut str = String::new();
        if self.is_final {
            str.push_str("final ");
        }
        str.push_str(self.view.to_java());
        str.push(' ');
        str.push_str(self.vartype.to_java());
        str.push(' ');
        str.push_str(self.name);
        str.push(';');

        str
    }
}

#[derive(Debug, Clone)]
pub struct Function<'a> {
    name: &'a str,
    view: View,
    returntype: Type<'a>,
    parameters: Vec<Attribute<'a>>,
    is_abstract: bool,
    is_static: bool,
}

impl<'a> Function<'a> {
    pub fn new(
        name: &'a str,
        view: View,
        returntype: Type<'a>,
        paremeters: Vec<Attribute<'a>>,
        is_abstract: bool,
        is_static: bool,
    ) -> Self {
        Self {
            name,
            view,
            returntype,
            parameters: paremeters,
            is_abstract,
            is_static,
        }
    }

    pub fn to_java(&self) -> String {
        let mut str = String::new();
        str.push_str(self.view.to_java());
        str.push(' ');
        if self.is_abstract {
            str.push_str("abstract");
            str.push(' ');
        }
        if self.is_static {
            str.push_str("static");
            str.push(' ');
        }
        str.push_str(self.returntype.to_java());
        str.push(' ');
        str.push_str(self.name);
        str.push('(');
        for p in self.parameters.iter() {
            str.push_str(&p.to_java_as_parameter());
        }
        if self.parameters.len() > 0 {
            str.pop();
            str.pop();
        }
        match self.is_abstract {
            true => str.push_str(");"),
            false => str.push_str(") {\n    }"),
        }

        str
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Type<'a> {
    Other(&'a str),
}

impl<'a> Type<'a> {
    pub fn to_java(&self) -> &'a str {
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
    pub fn to_java<'a>(&self) -> &'a str {
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
