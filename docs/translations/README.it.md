<p align="center">
  <img src="../../resources/icon.png" width="120" alt="AirDrop Bridge Logo" />
</p>

<h1 align="center">AirDrop Bridge</h1>

<p align="center">
  <strong>Trasferimenti di file nativi e ultrarapidi da iPhone e iPad a Windows e Linux.</strong><br>
  Nessuna app di terze parti su iOS, nessun account cloud e nessun abbonamento.
</p>

<p align="center">
  <a href="../../README.md">English</a> |
  <a href="README.es.md">Español</a> |
  <a href="README.zh.md">简体中文</a> |
  <a href="README.pt.md">Português</a> |
  <a href="README.fr.md">Français</a> |
  <a href="README.de.md">Deutsch</a> |
  <strong>Italiano</strong> |
  <a href="README.ja.md">日本語</a>
</p>

<p align="center">
  <a href="https://github.com/jorjanion/AirDrop-Bridge/releases"><img src="https://img.shields.io/github/v/release/jorjanion/AirDrop-Bridge?color=38bdf8&style=flat-square" alt="Release" /></a>
  <a href="https://github.com/jorjanion/AirDrop-Bridge/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue?style=flat-square" alt="Licenza" /></a>
  <img src="https://img.shields.io/badge/piattaforma-Windows%20%7C%20Linux-emerald?style=flat-square" alt="Piattaforma" />
  <img src="https://img.shields.io/badge/lingue-8%20integrate-purple?style=flat-square" alt="Lingue" />
  <img src="https://img.shields.io/badge/rust-edizione%202024-orange?style=flat-square" alt="Rust" />
</p>

> ⭐ **Ti piace AirDrop Bridge?** Lascia una stella su GitHub! Aiuta più persone a scoprire questo progetto gratuito e open source e ci incoraggia a migliorarlo continuamente.

---

## Panoramica

**AirDrop Bridge** ti consente di inviare foto, video 4K e documenti dal tuo iPhone o iPad direttamente al tuo computer utilizzando la rete Wi-Fi locale.

Invece di richiedere app esterne dall'App Store o caricare i tuoi file personali su server cloud di terze parti, AirDrop Bridge collega il tuo PC direttamente a un comando rapido ufficiale di Apple. Si integra perfettamente nel menu di condivisione di iOS, nel Tasto Azione e nel tocco posteriore.

<p align="center">
  <img src="../../assets/screenshots/pairing-window.png" width="280" alt="Finestra di associazione su PC" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="../../assets/screenshots/mobile-setup.png" width="240" alt="Portale di configurazione in Safari" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="../../assets/screenshots/ios-share-sheet.png" width="240" alt="Menu Condividi di iOS" />
</p>

---

## Caratteristiche Principali

* **100% Locale e Riservato**: Trasferimento peer-to-peer diretto via Wi-Fi. I tuoi file non lasciano mai la rete domestica.
* **Nessuna app di terze parti su iOS**: Utilizza un comando rapido ufficiale certificato da iOS.
* **Massima velocità**: Streaming a blocchi direttamente su disco con consumo di memoria costante (meno di 15 MB di RAM su PC). Trasferisci video pesanti senza rallentamenti.
* **Rilevamento senza configurazione**: Rilevamento automatico nella rete locale tramite mDNS con fallback su IP LAN.
* **Sicurezza predefinita**: Autorizzazione tramite token Bearer crittografico che impedisce trasferimenti non autorizzati su reti Wi-Fi pubbliche.
* **Supporto per 8 lingue**: Interfaccia completamente localizzata in English, Español, 简体中文, Português, Français, Deutsch, Italiano e 日本語.
* **Esperienza desktop nativa**: Finestra scura di associazione, icona nell'area di notifica, suono discreto e notifiche Windows.
* **Sincronizzazione degli appunti**: Condividi il testo copiato tra i dispositivi con un solo tocco.
* **Aggiornamenti automatici**: Un processo leggero in background ti assicura di avere sempre l'ultima versione.

<p align="center">
  <img src="../../assets/screenshots/tray-menus.png" width="620" alt="Menu e impostazioni nell'area di notifica" />
</p>

---

## Guida Rapida

La configurazione richiede meno di un minuto.

### 1. Scarica e avvia sul PC
* Scarica `airdrop-bridge-windows-x64.exe` dalla sezione [Releases di GitHub](https://github.com/jorjanion/AirDrop-Bridge/releases/latest).
* Fai doppio clic per avviare. Il programma si posizionerà nell'area di notifica e mostrerà la finestra con il codice QR.

### 2. Scansiona con la Fotocamera dell'iPhone
* Apri l'app Fotocamera sul tuo iPhone o iPad e inquadra il codice QR sullo schermo del PC.
* Tocca il link giallo per aprire il portale in Safari.

### 3. Installa il comando rapido Apple
* Nella pagina web, tocca **"1. Scarica e Installa il Comando AirDrop Bridge"**.
* Safari chiederà di aprire l'app Comandi Rapidi. Tocca **Apri** e poi seleziona **Aggiungi comando rapido**.

### 4. Collega il PC
* Torna in Safari e tocca **"2. Collega e Configura questo PC"**.
* Il comando salverà automaticamente il token di sicurezza e l'indirizzo IP del computer.

### 5. Inizia a condividere file e appunti!

#### Invio di Foto, Video e File:
* Apri l'app **Foto** o **File** sul tuo iPhone.
* Seleziona foto, video o documenti, tocca il pulsante **Condividi** e scegli **AirDrop Bridge**.
* Il file arriverà istantaneamente nella cartella Download (`Download\AirDropBridge`), accompagnato da un suono e da una notifica desktop!

#### Sincronizzazione degli Appunti (Bidirezionale):
* Avvia il comando **AirDrop Bridge** direttamente (dall'app Comandi Rapidi, da un widget della schermata Home, dal Tasto Azione o dal tocco posteriore).
* Apparirà un menu rapido con due opzioni:
  * 📤 **Send clipboard**: Invia il testo copiato sul tuo iPhone direttamente negli appunti del PC.
  * 📥 **Get clipboard**: Preleva il testo attualmente presente negli appunti del PC e lo copia sul tuo iPhone.
* Puoi anche selezionare qualsiasi testo sul tuo iPhone (in Safari, Note o Messaggi), toccare **Condividi** e scegliere **AirDrop Bridge** per inviarlo subito al computer!

---

## Avviso di Windows SmartScreen

Al primo avvio di AirDrop Bridge, Windows Defender SmartScreen potrebbe mostrare una finestra blu:

> *"PC protetto da Windows"*

### Perché compare questo avviso?
I certificati commerciali di firma del codice di Microsoft costano centinaia di euro ogni anno. Trattandosi di un progetto indipendente, gratuito e open source, non acquistiamo certificati aziendali a pagamento. Windows mostra temporaneamente questo messaggio sui nuovi file eseguibili fino a quando non accumulano un numero sufficiente di download.

### Come continuare:
1. Fai clic su **"Ulteriori informazioni"**.
2. Fai clic sul pulsante **"Esegui comunque"**.

L'intero codice sorgente è aperto al 100%, trasparente e consultabile direttamente in questo repository. Puoi anche compilarlo tu stesso da sorgente con Cargo se lo preferisci!

---

## Impostazioni nell'area di notifica

Fai clic con il tasto destro sull'icona di AirDrop Bridge nella barra delle applicazioni per accedere alle opzioni:

* **Apri AirDrop Bridge**: Mostra la finestra con il codice QR in qualsiasi momento.
* **Apri Download**: Accesso immediato alla cartella dei file ricevuti.
* **Copia Token**: Copia il token di sicurezza negli appunti.
* **Impostazioni**:
  * **Avvia con Windows**: Abilita o disabilita l'avvio automatico al login.
  * **Notifiche di Sistema**: Mostra o nasconde le notifiche desktop.
  * **Suono alla Ricezione**: Riproduce un suono discreto quando arriva un file.
  * **Apri Cartella alla Ricezione**: Apre automaticamente la cartella al termine del trasferimento.
  * **Sincronizza Appunti**: Mantiene attiva la condivisione del testo.
  * **Aggiornamenti Automatici**: Cerca e installa le nuove versioni in background.
  * **Lingua**: Passa all'istante tra le 8 lingue disponibili.

---

## Compilazione dal codice sorgente

Requisiti: [Rust](https://www.rust-lang.org/) (edizione 2024 o successiva).

```bash
git clone https://github.com/jorjanion/AirDrop-Bridge.git
cd airdrop-bridge
cargo build --release
```

L'eseguibile ottimizzato sarà generato in `target/release/airdrop-bridge.exe`.

---

## Contribuzioni, Segnalazioni (Issues) e PR

Accogliamo con entusiasmo segnalazioni di bug, idee e contributi dalla community!

### Segnalare problemi o richiedere funzionalità
Se riscontri un problema o hai una proposta:
1. Visita la sezione [Issues di GitHub](https://github.com/jorjanion/AirDrop-Bridge/issues).
2. Controlla se l'argomento è già stato trattato.
3. Fai clic su **New Issue** e specifica:
   * Cosa è accaduto e cosa ti aspettavi succedesse.
   * La versione del tuo sistema operativo (es. Windows 11 o distribuzione Linux).
   * La versione di iOS/iPadOS (es. iOS 18.1).
   * I passaggi per riprodurre il problema.

### Inviare una Pull Request (PR)
1. Fai un fork del repository e crea un branch dedicato:
   ```bash
   git checkout -b feature/la-mia-modifica
   ```
2. Applica le modifiche e assicurati che tutti i test passino:
   ```bash
   cargo test
   ```
3. Crea i commit seguendo le convenzioni standard (es. `feat: ...`, `fix: ...`).
4. Esegui il push sul tuo fork e apri una Pull Request verso il branch `main`.
5. Descrivi brevemente le modifiche introdotte. Esaminiamo e integriamo i contributi attivamente!

---

## Licenza

AirDrop Bridge è distribuito a tua scelta sotto una delle seguenti licenze:

* Licenza Apache, Versione 2.0 ([LICENSE-APACHE](../../LICENSE) o http://www.apache.org/licenses/LICENSE-2.0)
* Licenza MIT ([LICENSE-MIT](../../LICENSE) o http://opensource.org/licenses/MIT)
