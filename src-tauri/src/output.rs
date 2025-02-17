use genpdf::elements::{Text, TableLayout, FrameCellDecorator};
use genpdf::{fonts, Document, style::Style, Element, Margins};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Information {
    id: String,
    title: String,
    date: String,
    total: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Client {
    name: String,
    dni: String,
    phone: String,
    concept: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Vehicle {
    plate: String,
    maker: String,
    model: String,
    kilometrage: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Detail {
    item: String,
    price: String,
    cant: String,
    tipo: String,
    subtotal: String,
    iva: String,
    total: String
}

pub fn write_pdf(info: Information, client: Client, vehicle: Vehicle, details: Vec<Detail>) -> Result<String, String> {
    let font = fonts::from_files("./src", "Montserrat", None).expect("");
    let mut doc = Document::new(font);
    doc.set_title(format!("{} - {}", client.name, info.id));

    // Decorador del archivo
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    // PDF header
    let mut header_style = Style::new();
    header_style.set_bold();
    header_style.set_font_size(20);
    doc.push(Text::new(format!("{} N° {}", info.title, info.id)).styled(header_style));

    let mut date_style = Style::new();
    date_style.set_font_size(10);
    doc.push(Text::new(format!("Fecha: {}", info.date)).styled(date_style));

    let mut company_style = Style::new();
    company_style.set_bold();
    doc.push(Text::new("Punto Diesel - Rqta").styled(company_style));

    // Customer data
    let customer_margin = Margins::trbl(15.0, 0.0, 10.0, 0.0);
    let mut title_style = Style::new();
    title_style.set_bold();
    let mut customer_table = TableLayout::new(vec![1, 1]);

    let customer_row = vec![
        Box::new(Text::new("Datos del cliente:").styled(title_style)) as Box<dyn Element>,
        Box::new(Text::new("Datos del vehiculo:").styled(title_style)) as Box<dyn Element>,
    ];
    customer_table.push_row(customer_row).expect("");

    customer_table.push_row(vec![
        Box::new(Text::new(format!("Nombre: {}", client.name))) as Box<dyn Element>,
        Box::new(Text::new(format!("Dominio: {}", vehicle.plate))) as Box<dyn Element>,
    ]).expect("");
    customer_table.push_row(vec![
        Box::new(Text::new(format!("DNI: {}", client.dni))) as Box<dyn Element>,
        Box::new(Text::new(format!("Marca: {}", vehicle.maker))) as Box<dyn Element>,
    ]).expect("");
    customer_table.push_row(vec![
        Box::new(Text::new(format!("Telefono: {}", client.phone))) as Box<dyn Element>,
        Box::new(Text::new(format!("Modelo: {}", vehicle.model))) as Box<dyn Element>,
    ]).expect("");
    customer_table.push_row(vec![
        Box::new(Text::new(format!("Concepto: {}", client.concept))) as Box<dyn Element>,
        Box::new(Text::new(format!("Kilometraje: {}", vehicle.kilometrage))) as Box<dyn Element>,
    ]).expect("");

    let padded_table = customer_table.padded(customer_margin);
    doc.push(padded_table);

    doc.push(Text::new("Detalles:").styled(title_style).padded(Margins::trbl(0,0,2,0)));
    
    // Define styles for Details Table
    let mut details_table = TableLayout::new(vec![5, 2, 1, 2, 2, 1, 2]);
    details_table.set_cell_decorator(FrameCellDecorator::new(true, true, false));

    // Custom margin
    let details_margin = Margins::trbl(1, 0.1, 2, 1);

    // Add a first row
     let header_row = vec![
        Box::new(Text::new("Item").padded(details_margin)) as Box<dyn Element>,
        Box::new(Text::new("Precio u.").padded(details_margin)) as Box<dyn Element>,
        Box::new(Text::new("Cant").padded(details_margin)) as Box<dyn Element>,
        Box::new(Text::new("Subtotal").padded(details_margin)) as Box<dyn Element>,
        Box::new(Text::new("Tipo").padded(details_margin)) as Box<dyn Element>,
        Box::new(Text::new("IVA").padded(details_margin)) as Box<dyn Element>,
        Box::new(Text::new("Total").padded(details_margin)) as Box<dyn Element>,
    ];
     
     details_table.push_row(header_row).expect("");

    // Add row for each item of list
    for detail in details.iter() {
        let data_row = vec![
            Box::new(Text::new(&detail.item).padded(details_margin)) as Box<dyn Element>,
            Box::new(Text::new(detail.price.to_string().replace(".",",")).padded(details_margin)) as Box<dyn Element>,
            Box::new(Text::new(detail.cant.to_string()).padded(details_margin)) as Box<dyn Element>,
            Box::new(Text::new(detail.subtotal.to_string().replace(".",",")).padded(details_margin)) as Box<dyn Element>,
            Box::new(Text::new(&detail.tipo).padded(details_margin)) as Box<dyn Element>,
            Box::new(Text::new(detail.iva.to_string().replace(".",",")).padded(details_margin)) as Box<dyn Element>,
            Box::new(Text::new(detail.total.to_string().replace(".",",")).padded(details_margin)) as Box<dyn Element>,
        ];
        details_table.push_row(data_row).expect("");
    }
    doc.push(details_table);

    // Pay Table
    let mut pay_table = TableLayout::new(vec![1, 1]);
    pay_table.set_cell_decorator(FrameCellDecorator::new(true, true, false));
    pay_table.push_row(vec![
        Box::new(Text::new("Total: ").padded(details_margin)) as Box<dyn Element>,
        Box::new(Text::new(info.total.replace(".",",")).padded(details_margin)) as Box<dyn Element>
    ]).expect("");
    let padded_pay = pay_table.padded(Margins::trbl(3, 0, 0, 0));
    doc.push(padded_pay);

    // Guardar el PDF
    match doc.render_to_file(format!("{} - {}.pdf", client.name, info.id)) {
        Ok(_) => Ok(format!("PDF of {} N° {} saved successfully", info.title, info.id)),
        Err(_) => Err(format!("PDF of {} N° {} not created.", info.title, info.id))
    }
}
