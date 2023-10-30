use thirtyfour::prelude::*;
use thirtyfour::error::WebDriverResult;
use std::time::Duration;
use libs:: {sleep, sleep_ms};
use std::path::Path;

// sisteme giris icazesin verildiyini yoxla

async fn check_id(driver: &WebDriver) -> WebDriverResult<String> {
    // Elementin görüntülenmesini beklemek için bir döngü kullanabilirsiniz.
    let elem_form = loop {
        if let Ok(elem) = driver.find(By::Css("#app > div")).await {
            break elem;
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    };

    let elem_form_content = elem_form.text().await?;
    Ok(elem_form_content)
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut ch_selector = 0;
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.goto("https://web.whatsapp.com").await?;

    // Elementin görüntülenmesini beklemek için bir döngü kullanabilirsiniz.
    let mut elem_form = loop {
        if let Ok(elem) = driver.find(By::Css("#app > div")).await {
            break elem;
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    };

    let elem_form_content = check_id(&driver).await?;
    let control_text = "Use WhatsApp on your computer";
    while elem_form_content.contains(control_text) {
        let check = check_id(&driver).await?;
        println!("QR kodu okutup tekrar kontrol edin");
        driver.refresh().await?;
        tokio::time::sleep(Duration::from_secs(20)).await;
        if !check.contains(control_text) {
            println!("Loop exit");
            break;
        } else {
            println!("Loop continue");
        }
    }
    driver.maximize_window().await?;
    tokio::time::sleep(Duration::from_secs(5)).await;

    sleep_ms(200);
    let mut counter = 0;
    let mut numbers = String::new(); // veya String::from("") ile başlatabilirsiniz

    while true {
        println!("Sira: {}", counter);
        if counter == 50 {
            break;
        }
        let mut new_chat = loop {
            if let Ok(elem) = driver.find(By::Css("#app > div > div > div._2Ts6i._3RGKj > header > div._604FD > div > span > div:nth-child(4) > div > span")).await {
                break elem;
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        };
        new_chat.click().await?;

        sleep_ms(1000);
        libs::simulate_typing("metros");
        for i in 0..=2 {
            sleep_ms(1000);
            libs::Tab();
        }
    
              
        for i in 0..=counter {
            sleep_ms(100);
            libs::step();
        }

        sleep_ms(200);
        libs::enter();

        sleep(1);
        
        let mut profil = loop {
            println!("Profil Yuklenir");
            if let Ok(elem) = driver.find(By::Css("#main > header > div._2au8k > div > div")).await {
                break elem;
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        };
        profil.click().await?;
        sleep_ms(200);

        let mut number = loop {
            println!("Nomre Yuklenir");
            if ch_selector > 1 {
                ch_selector = 0;
            }
            let css_ch: [&str; 2] = ["#app > div > div > div._2Ts6i._1xFRo > span > div > span > div > div > section > div.gsqs0kct.oauresqk.efgp0a3n.tio0brup.g0rxnol2.tvf2evcx.oq44ahr5.lb5m6g5c.brac1wpa.lkjmyc96.i4pc7asj.bcymb0na.przvwfww.e8k79tju > div.gx1rr48f._3VUan > div > div > span > span",
                                     "#app > div > div > div._2Ts6i._1xFRo > span > div > span > div > div > section > div.gsqs0kct.oauresqk.efgp0a3n.tio0brup.g0rxnol2.tvf2evcx.oq44ahr5.lb5m6g5c.brac1wpa.lkjmyc96.b8cdf3jl.bcymb0na.myel2vfb.e8k79tju > div.tvf2evcx.m0h2a7mj.lb5m6g5c.j7l1k36l.ktfrpxia.nu7pwgvd.p357zi0d.dnb887gk.gjuq5ydh.i2cterl7.fhf7t426.f8m0rgwh.gndfcl4n > div > span > span"];

            if let Ok(elem) = driver.find(By::Css(css_ch[ch_selector])).await {
                break elem;
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
            ch_selector +=1;
        };
        sleep_ms(100);
        println!("{}\n", number.text().await?);
        let name:String = format!("image/screenshots/screenshot-{}.png", counter);
        let screenshot_data = driver.screenshot(Path::new(&name)).await?;
        sleep_ms(100);
        counter+=1;
    }
    
    Ok(())
}
