use super::Form;

#[derive(Debug, Clone)]
pub struct Call {
    pub callable: Box<Form>, 
    pub arguments: Vec<Form>
}

impl Call {
    pub fn new(callable: Form, arguments: Vec<Form>) -> Self {
        Self { callable: Box::new(callable), arguments }
    }

    pub fn callable(&self) -> &Box<Form> {
        &self.callable
    }

    pub fn arguments(&self) -> &Vec<Form> {
        &self.arguments
    }
}