#  Mitt assembly språk

Ideen kom fra YoutTube videon [Machine Code Explained](https://www.youtube.com/watch?v=8VsiYWW9r48&t=500s) av kanalen Computerphile. I videon hadde de funnet opp et veldig enkelt assembly språk som de brukte på papir, men de viste aldri eksempler digitalt på en datamaskin. Da kom ideen om jeg må lage en interpreter til et lignende språk.
Jeg valgte å brule programeringsspråket Rust fordi jeg har holdt mye på med det i det siste. Rust er laget for å være veldig raskt og helt minnesikkert uten å ha en garbage collector eller runtime, men det kommer på bekostning av kompleksitet. 

## Assembly
Assembly er maskinkode i en form som er mulig å lese. Maskinkode er enerene og nullerene datamaskinen kjører. Alle programmer kjøres som maskinkode på datamaskinen, men man skriver ikke i assenbly lenger, unntatt i veldig spesifike situasjoner. Alle progarmeringsspråk gjøres om til maskinkode før det blir kjørt.

## Mitt språk
Mitt språk består av et lite sett med instruksjoner som kan ta inn forskjellige parametrer
<br/><br/>
**Her er et eksempel på et program som regner ut Fibonacci:**
```Assembly
Load 1
Store @0
Store @1
Load @Index
Inc
Add @Index
Inc
Store @Index
Dec
Jump #3
```
Legg merke til at det er bare nullere i slutten av minne. Det er på grunn av at tallet blir større enn den kan lagre. Alle error blir printet ut til developer konsollen, inkludert dette erroret.
<br/><br/>
**Deler av språket som er viktig å kjenne til**
- Index - En variabel som peker på en minneadresse
- Accumulator - Accumulatoren er en variabel som brukes til


**Dette er instruksjonene som er tilgjengelig**
- `Add` - Øker accumulatoren med det som står i parameteret
- `Store` - Lagrer det som står i accumulatoren i minneadressen som står i parameteret
- `Load` - Setter accumulatoren til det den får i parameteret
- `Inc` - Øker indexen med 1
- `Dec` Minsker indexen med 1
- `Jump` Kjører instruksen på linjen den får som parameter (Husk at man starter på 0)

**Parametrer**
- `@(minneadresse)` - Minneadresse - "@" så en minneadresse
- `(tall)` - Konst - Bare et tall
- `#(linjenummer)` - Linjenummer - Brukes bare på `Jump`
- `@Index` - Minne på adresse Index - Ligner på parameteret minneadresse, men brukes variabelen Index istedenfor et tall

**Ikke alle forskjellige paramatrer kan brukes på alle instruksjoner.**
- `Add` - Konstant eller minneadresse
- `Store` - Minneadresse
- `Load` - Minneadresse eller konst
- `Inc` - Ingen
- `Dec` - Ingen
- `Jump` - Linjenummer
