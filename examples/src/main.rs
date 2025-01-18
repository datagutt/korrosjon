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

    offentlig(eske) funksjon kanskje(i: u32) -> Kanskje<Resultat<u32, Streng>> {
        hvis i % 2 == 1 {
            hvis i == 42 {
                Noen(Prob(Streng::fra("merav")))
            } ellers {
                Noen(Bra(33))
            }
        } ellers {
            Ingenting
        }
    }

    asynkron funksjon eksempel() {
    }

    asynkron funksjon eksempel2() {
        eksempel().avvent;
    }

    funksjon hoved() {
        la foranderlig x = 31;

        sammenlign x {
            42 => {
                skrivlinje!("vaffel med brunost")
            }
            _ => skrivlinje!("jepsipepsi")
        }

        for i av 0..10 {
            la verdi = løkke {
                bryt i;
            };

            imens ingen x < verdi {
                x += 1;
            }

            x = hvis la Noen(resultat) = kanskje(i) {
                resultat.pakk_opp()
            } ellers {
                12
            };
        }

        sekundær();
    }

    #[tillat(uoppnåelig_kode)]
    funksjon sekundær() {
        panikk!("å nei");
        dritt!("drittkoav");
        faen!("fy faen");
        ops!("ops, avtte gikk galt");
        uff!("uff, avtte var ikke bra");
    }
}
