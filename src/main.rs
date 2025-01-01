use reqwest;
use scraper::{Html, Selector};
use std::fs::OpenOptions;
use std::io::{self, Write};
use chrono::Local;

fn main() -> io::Result<()> {
    let url = "https://www.amazon.in/Lenovo-7235HS-300Nits-Graphics-83JC0031IN/dp/B0D7VMYDJY/ref=sr_1_4?crid=112782J50VD8C&dib=eyJ2IjoiMSJ9.8yTKZyM5JrYQiGKoFWCwtHLwd39_08_OvK06UTkPXKRY-98JqkAcb8EfEN4fJOJ5h6gcbChH33-zxzQWPnnTtuGXSb3Dv1hcG1sw3YDGm1SasgJguYHnsZIW_3bexG43UFZ7SvkVIK7nyy-WQpDgT-kCjavdXuHxs2g3idGlCOokh6D10ScQKSW6N1XtV10z_y1SE1-zCaIOAgldKA31PVFUZYcop_pWo2bekB3jBJANOhbSS9wZ_GGCN6JpSbVt8tq1hjlN-PEwVtnzFeGd0pdF-cFMf5ZmpOLN8bbWpa4fye3vbTw7v-OiY9W4iQEjwKpVLlROoeovWNvsZ0eLq9AUnTsMNayNuvm_KdaPcqI.Nw0VlhB2du41QFIg1A7AlvgPSNaddggQXdWKWOQEjww&dib_tag=se&keywords=gaming+laptop&qid=1735719059&s=computers&sprefix=gaming%2Ccomputers%2C465&sr=1-4";
    let res = reqwest::blocking::get(url).unwrap().text().unwrap();
    let document = Html::parse_document(&res);

    let title = document.select(&Selector::parse("span#productTitle").unwrap()).next().unwrap().text().collect::<Vec<_>>().join("");
    let price = document.select(&Selector::parse("span.a-price-whole").unwrap()).next().unwrap().text().collect::<Vec<_>>().join("");

    let timestamp = Local::now().to_rfc3339();

    println!("Product Title: {}", title);
    println!("Price: {}", price);
    println!("Timestamp: {}", timestamp);

    let product_id = url.split("/dp/").nth(1).unwrap_or("unknown").split("/").next().unwrap_or("unknown");
    let file_name = format!("price_history_{}.csv", product_id);

    writeln!(OpenOptions::new().create(true).append(true).open(file_name)?, "{},{},{}", title, price, timestamp)?;

    println!("Price history updated successfully!");

    Ok(())
}
