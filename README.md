# korrosjon

Are you tired of writing Rust programs in English? Do you find yourself craving to shout out Norwegian expressions while coding?
Looking for a new, exotic-sounding language that will bring some Scandinavian flair to your Rust projects?

**korrosjon** (Norwegian for _Rust_) is here to save your day, as it allows you to
write Rust programs entirely in Norwegian, complete with Norwegian keywords, function names, and idioms.

Worried about being stuck with only Norwegian words, especially if you’re from a place where Norwegian isn’t the only lingua franca?

No worries! Norwegian Rust is fully compatible with English Rust, so you can mix both languages as you please.

Here's an example of what can be achieved with Rouille:

Below is an example showcasing what is possible with korrosjon:

### egenskap og impl (aka egenskap og implementasjon)

```rust
korrosjon::korrosjon! {
    ekstern eske korrosjon;

    bruk std::collections::Ordbok som Ordbok;

    egenskap Nøkkelverdi {
        funksjon skriv(&selv, nøkkel: Streng, verdi: Streng);
        funksjon les(&selv, nøkkel: Streng) -> Resultat<Kanskje<&Streng>, Streng>;
    }

    statisk foranderlig ORDBOK: Kanskje<Ordbok<Streng, Streng>> = Ingenting;

    struktur Konkret;

    implementer Nøkkelverdi for Konkret {
        funksjon skriv(&selv, nøkkel: Streng, verdi: Streng) {
            la ordbok = utrygt {
                ORDBOK.ta_eller_sett_inn_med(Standard::standard)
            };
            ordbok.sett_inn(nøkkel, verdi);
        }
        funksjon les(&selv, nøkkel: Streng) -> Resultat<Kanskje<&Streng>, Streng> {
            hvis la Noen(ordbok) = utrygt { ORDBOK.som_ref() } {
                Bra(ordbok.les(&nøkkel))
            } ellers {
                Prob("henting av ordbok".til())
            }
        }
    }
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax.
_Det er ikke verre en det_

## Want to help?

First of all, _tusen takk_ for considering participating to this **completely serious** project. Feel free to throw in a few identifiers
here and there, and open a pull-request against the `hoved` (Norwegian for
`main`) branch.

We would love support for Norwegian Nynorsk as an optional feature in the crate.

## Other languages

- French: [rouille](https://github.com/bnjbvr/rouille)
- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [Ржавый](https://github.com/Sanceilaks/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
- Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
- Spanish: [rustico](https://github.com/UltiRequiem/rustico)
- Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
- Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
- Arabic: [sada](https://github.com/LAYGATOR/sada)
- Turkish: [pas](https://github.com/ekimb/pas)
- Vietnamese: [gỉ](https://github.com/Huy-Ngo/gir)
- Japanese: [sabi (錆)](https://github.com/yuk1ty/sabi)
- Danish: [rust?](https://github.com/LunaTheFoxgirl/rust-dk)
- Marathi: [gan̄ja](https://github.com/pranavgade20/ganja)
- Romanian: [rugină](https://github.com/aionescu/rugina)
- Czech: [rez](https://github.com/radekvit/rez)
- Ukrainian: [irzha](https://github.com/brokeyourbike/irzha)
- Bulgarian: [ryzhda](https://github.com/gavadinov/ryzhda)
- Slovak: [hrdza](https://github.com/TheMessik/hrdza)
- Catalan: [rovell](https://github.com/gborobio73/rovell)
- Corsican: [rughjina](https://github.com/aldebaranzbradaradjan/rughjina)
- Indonesian: [karat](https://github.com/annurdien/karat)
- Lithuanian: [rūdys](https://github.com/TruncatedDinosour/rudys)
- Greek: [skouriasmeno](https://github.com/devlocalhost/skouriasmeno)
- Thai: [sanim (สนิม)](https://github.com/korewaChino/sanim)
- Swiss: [roeschti](https://github.com/Georg-code/roeschti)
- Swedish: [rost](https://github.com/vojd/rost/)
- Croatian: [hrđa](https://github.com/njelich/hrdja)
- Persian: [zangar (زنگار)](https://github.com/ui-ce/zangar)
- Malagasy: [arafesina](https://github.com/luckasRanarison/arafesina)
- Latin: [ferrugo](https://github.com/pianoman911/ferrugo)
- All of the above: [unirust](https://github.com/charyan/unirust)

## license

[WTFPL](http://www.wtfpl.net/)
