


struct kullanici{
    isim: String,
    yas: u32,
    email: String,
    sifre: u32,

}



fn main() {

    let kullanici1= kullanici{
        isim: String::from("nafia zeynep dönmez"),
        yas: 20,
        email: String::from("nf.zynp08@gmail.com"),
        sifre: 12345
    
    };

    let kullanici2: kullanici= kullanici {
        isim: String::from("taha dönmez"),
        yas: 13,
        email: String::from("taha25@gmail.com"),
        sifre:56789
    };

   

    
    


    println!("kullanici adi: {}",kullanici1.isim);
    println!("kullanici yasi: {}",kullanici1.yas);
    println!("kullanici e postasi: {}",kullanici1.email);
    println!("kullanici sifre:{}",kullanici1.sifre);

    println!("kullanici adi:{}",kullanici2.isim);
    println!("kullanici yasi:{}",kullanici2.yas);
    println!("kullanici e postasi:{}",kullanici2.email);
    println!("kullanici sifre:{}",kullanici2.sifre);



    


}


