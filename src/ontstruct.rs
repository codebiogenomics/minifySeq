/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-8-7
*/

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Clean {
    pub id: String,
    pub start: Vec<usize>,
    pub end: Vec<usize>,
}
