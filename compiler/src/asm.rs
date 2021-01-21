pub struct DataSectionItem {
    name: String,
    size: String,
    values: String,
}

pub struct DataSection {
    items: Vec<DataSectionItem>,
}

impl Default for DataSection {
    fn default() -> Self {
        DataSection { items: Vec::new() }
    }
}

impl DataSection {
    pub fn new() -> Self {
        DataSection::default()
    }

    pub fn append<N, S, V>(&mut self, name: N, size: S, values: V)
    where
        N: Into<String>,
        S: Into<String>,
        V: Into<String>,
    {
        self.items.push(DataSectionItem {
            name: name.into(),
            size: size.into(),
            values: values.into(),
        });
    }
}

pub enum TextSectionItem {
    Label(String),
    Instruction(String),
}

pub struct TextSection {
    items: Vec<TextSectionItem>,
}

impl Default for TextSection {
    fn default() -> Self {
        TextSection { items: Vec::new() }
    }
}

impl TextSection {
    pub fn new() -> Self {
        TextSection::default()
    }

    pub fn label<N: Into<String>>(&mut self, name: N) {
        self.items.push(TextSectionItem::Label(name.into()));
    }

    pub fn inst<I: Into<String>>(&mut self, inst: I) {
        self.items.push(TextSectionItem::Instruction(inst.into()));
    }

    pub fn extend(&mut self, other: TextSection) {
        self.items.extend(other.items)
    }
}

pub struct Asm {
    pub data: DataSection,
    pub text: TextSection,
}

impl Asm {
    pub fn stringify(&self) -> String {
        let mut result = String::from("bits 64\nglobal _start\n\nsection .data\n");
        for item in self.data.items.iter() {
            result.push_str(format!("    {} {} {}\n", item.name, item.size, item.values).as_str());
        }
        result.push_str("\nsection .text\n");
        for item in self.text.items.iter() {
            match item {
                TextSectionItem::Label(label_name) => {
                    result.push_str(format!("{}:\n", label_name).as_str());
                }
                TextSectionItem::Instruction(inst) => {
                    result.push_str(format!("    {}\n", inst).as_str());
                }
            }
        }
        result
    }
}