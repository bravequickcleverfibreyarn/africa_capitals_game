use noiserand::NoiseRand;
use rand_core::RngCore;
use std::io::{stdin, stdout, Write};

type DataItem = (&'static str, &'static str, &'static [&'static str]);

// Possible new features:
//    ⸻ Guessing country: by flag, by capital.

const COUNTRIES: usize = 54;
#[rustfmt::skip]
static mut MAY_2024: [DataItem; COUNTRIES] = [
    ("Algeria", "🇩🇿", &["Algiers"]),
    ("Angola", "🇦🇴", &["Luanda"]),
    ("Benin", "🇧🇯", &["Porto-Novo"]),
    ("Botswana", "🇧🇼", &["Gaborone"]),
    ("Burkina Faso", "🇧🇫", &["Ouagadougou"]),
    ("Burundi", "🇧🇮", &["Gitega"]),
    ("Cameroon", "🇨🇲", &["Yaounde"]),
    ("Cape Verde", "🇨🇻", &["Praia"]),
    ("Central African Republic", "🇨🇫", &["Bangui"]),
    ("Chad", "🇹🇩", &["N'Djamena"]),
    ("Comoros", "🇰🇲", &["Moroni"]),
    ("Democratic Republic of the Congo", "🇨🇩", &["Kinshasa"]),
    ("Republic of the Congo", "🇨🇬", &["Brazzaville"]),
    ("Djibouti", "🇩🇯", &["Djibouti"]),
    ("Egypt", "🇪🇬", &["Cairo"]),
    ("Equatorial Guniea", "🇬🇶", &["Malabo"]),
    ("Eritrea", "🇪🇷", &["Asmara"]),
    ("Eswatini", "🇸🇿", &["Mbabane", "Lobamba"]),
    ("Ethiopia", "🇪🇹", &["Addis Ababa"]),
    ("Gabon", "🇬🇦", &["Libreville"]),
    ("Gambia", "🇬🇲", &["Banjul"]),
    ("Ghana", "🇬🇭", &["Accra"]),
    ("Guinea", "🇬🇳", &["Conakry"]),
    ("Guinea-Bissau", "🇬🇼", &["Bissau"]),
    ("Ivory Coast", "🇨🇮", &["Yamoussoukro"]),
    ("Kenya", "🇰🇪", &["Nairobi"]),
    ("Lesotho", "🇱🇸", &["Maseru"]),
    ("Liberia", "🇱🇷", &["Monrovia"]),
    ("Libya", "🇱🇾", &["Tripoli"]),
    ("Madagascar", "🇲🇬", &["Antananarivo"]),
    ("Malawi", "🇲🇼", &["Lilongwe"]),
    ("Mali", "🇲🇱", &["Bamako"]),
    ("Mauritania", "🇲🇷", &["Nouakchott"]),
    ("Mauritius", "🇲🇺", &["Port Louis"]),
    ("Morocco", "🇲🇦", &["Rabat"]),
    ("Mozambique", "🇲🇿", &["Maputo"]),
    ("Namibia", "🇳🇦", &["Windhoek"]),
    ("Niger", "🇳🇪", &["Niamey"]),
    ("Nigeria", "🇳🇬", &["Abuja"]),
    ("Rwanda", "🇷🇼", &["Kigali"]),
    ("Sao Tome and Principe", "🇸🇹", &["Sao Tome"]),
    ("Senegal", "🇸🇳", &["Dakar"]),
    ("Seyschelles", "🇸🇨", &["Victoria"]),
    ("Sierra Leone", "🇸🇱", &["Freetown"]),
    ("Somalia", "🇸🇴", &["Mogadishu"]),
    ( "South Africa", "🇿🇦", &["Pretoria", "Cape Town", "Bloemfontein"],),
    ("South Sudan", "🇸🇸", &["Juba"]),
    ("Sudan", "🇸🇩", &["Khartoum"]),
    ("Tanzania", "🇹🇿", &["Dodoma"]),
    ("Togo", "🇹🇬", &["Lome"]),
    ("Tunisia", "🇹🇳", &["Tunis"]),
    ("Uganda", "🇺🇬", &["Kampala"]),
    ("Zambia", "🇿🇲", &["Lusaka"]),
    ("Zimbabwe", "🇿🇼", &["Harare"]),
];

#[derive(PartialEq, Copy, Clone)]
enum HintAmount {
    None,
    Full,
    CountryOnly,
}

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    let args: &[&str] = &args.iter().map(|x| x.as_str()).collect::<Vec<&str>>();

    let mut stdout = stdout();

    let mut colorize = true;
    let mut flag_mode = false;
    let mut hint_amount = HintAmount::Full;

    if args.contains(&"--help") {
        #[rustfmt::skip]writeln!(&mut stdout, "\n--- HELP ---\n")?;
        #[rustfmt::skip]writeln!(&mut stdout, "    --help              |this help")?;
        #[rustfmt::skip]writeln!(&mut stdout, "    --nocolor           |no output colorization")?;
        #[rustfmt::skip]writeln!(&mut stdout, "    --list              |outputs country list")?;
        #[rustfmt::skip]writeln!(&mut stdout, "    --flag-only         |flag only mode")?;
        #[rustfmt::skip]writeln!(&mut stdout, "    --hint:none         |provides no hint on error")?;
        #[rustfmt::skip]writeln!(&mut stdout, "    --hint:country-only |only country hint, use with --flag-only\n")?;

        stdout.flush()?;
        return Ok(());
    }

    if args.contains(&"--list") {
        writeln!(&mut stdout, "┎{:━<35}┮━━━━┭{:━<35}┒", "", "")?;
        writeln!(&mut stdout, "│{:^35}┃Flag┃{:^35}│", "Country", "Capital")?;
        writeln!(&mut stdout, "┞{:━<35}╆━━━━╅{:━<35}┦", "", "")?;

        let mut inx = 0;
        unsafe {
            loop {
                let it = MAY_2024[inx];

                writeln!(
                    &mut stdout,
                    "│{:<35}│{:^4}│ {:<33} │",
                    it.0,
                    it.1,
                    it.2.join(", ")
                )?;
                inx += 1;

                if inx < COUNTRIES {
                    writeln!(&mut stdout, "┞{:─<35}╅────╆{:─<35}┦", "", "")?;
                } else {
                    break;
                }
            }
        }
        writeln!(&mut stdout, "┗{:━<35}┶━━━━┵{:━<35}┛", "", "")?;
        stdout.flush()?;
        return Ok(());
    }

    if args.contains(&"--flag-only") {
        flag_mode = true;
    }

    if args.contains(&"--nocolor") {
        colorize = false;
    }

    if args.contains(&"--hint:none") {
        hint_amount = HintAmount::None
    } else if args.contains(&"--hint:country-only") {
        if flag_mode {
            hint_amount = HintAmount::CountryOnly
        }
    }

    writeln!(
        &mut stdout,
        "\n----> Welcome to the Africa Capitals Game <----\n"
    )?;
    writeln!(
        &mut stdout,
        "Acquiring quantum fluctuations based seed. Check https://qrng.anu.edu.au/ for more.\n\n"
    )?;
    stdout.flush()?;

    let mut nr = NoiseRand::new();
    let rn = nr.next_u32();
    let b0 = rn.to_ne_bytes()[0];

    const LT_INX: usize = COUNTRIES - 1;
    
    let mut ix1 = 0;
    let mut ix2 = match b0 as usize {
        x if x > LT_INX => x % COUNTRIES,
        x => x,
    };

    while ix2 < COUNTRIES {
        unsafe {
            let swap = MAY_2024[ix2];
            MAY_2024[ix2] = MAY_2024[ix1];
            MAY_2024[ix1] = swap;
        }
        ix1 += 1;
        ix2 += 1;
    }

    for i in 0..LT_INX {
        ix1 = i;
        ix2 = LT_INX;
        while ix1 < ix2 {
            unsafe {
                let swap = MAY_2024[ix2];
                MAY_2024[ix2] = MAY_2024[ix1];
                MAY_2024[ix1] = swap;
            }

            ix1 += 1;
            ix2 -= 1;
        }
    }

    let mut buff = String::with_capacity(2);
    let num = loop {
        write!(&mut stdout, "Tell batch size [1-54]: ")?;
        stdout.flush()?;
        buff.clear();

        read_line(&mut buff);
        let try_num = buff.parse::<u8>();

        if let Ok(num) = try_num {
            let num = num as usize;
            if num > 0 && num < COUNTRIES + 1 {
                break num;
            }
        }

        writeln!(
            &mut stdout,
            "   ┖━━━━╾ Error ━╾ {}",
            colorized(colorize, &buff, "\x1b[0;43m")
        )?;
    };

    let mut inx = 0;
    while inx < num {
        let item = unsafe { MAY_2024[inx] };

        let mut country = String::from(item.1);
        if !flag_mode {
            country.insert_str(0, ", ");
            country.insert_str(0, item.0);
        }

        writeln!(&mut stdout, "Capital of {}?", country)?;
        stdout.flush()?;

        buff.clear();
        read_line(&mut buff);

        let (mut print, mut color) = ("Never", "\x1b[0;31m");
        for &c in item.2 {
            let buff = &buff;
            if c == buff {
                inx += 1;
                (print, color) = ("Yes", "\x1b[0;32m")
            }
        }

        let mut hint = String::from("");
        match hint_amount {
            HintAmount::Full => {
                hint.push_str(", ");
                let join = item.2.join(", ");
                hint.push_str(&join);
                if flag_mode {
                    hint.push_str(", ");
                    hint.push_str(item.0);
                }
            }
            HintAmount::CountryOnly => {
                hint.push_str(". Country: ");
                hint.push_str(item.0);
            }
            HintAmount::None => {}            
        }

        writeln!(
            &mut stdout,
            "{}{}.\n",
            colorized(colorize, print, color),
            hint
        )?;
    }

    stdout.flush()?;
    Ok(())
}

fn read_line<'a>(buff: &'a mut String) -> &'a str {
    while let Err(_) = stdin().read_line(buff) {}
    *buff = buff.replace("\n", "").replace("\r", "");
    buff.trim_start().trim_end()
}

fn colorized(colorize: bool, txt: &str, color: &str) -> String {
    let mut txt = txt.to_string();
    if colorize {
        txt.insert_str(0, color);
        txt.push_str("\x1b[0;0m");
    }

    txt
}
