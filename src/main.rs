use anyhow::{Context, Result, bail};
use wasmtime::*;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn save_precompiled_file(buffer: &[u8]) -> std::io::Result<()> {
    println!("Creating File...");
    let mut precompiled_module = File::create("add.cwasm")?;
    println!("Saving File...");
    precompiled_module.write_all(&buffer)?;
    Ok(())
}

fn load_precompiled_file(engine: &Engine, path: &Path) -> Result<Module> {
    let buffer = fs::read("./wasm/add.wat");
    let module =  unsafe { Module::deserialize_file(engine, path) };
    module
}

fn main() -> Result <()> {
    //Der erste Schritt ist die Kompilierung:
    println!("Kompilieren ...");

    let mut config = Config::new();
    config.interruptable(true);
    config.static_memory_maximum_size(1024000);
    config.static_memory_guard_size(512000);
    config.debug_info(true);
    config.consume_fuel(true);

    let engine = Engine::new(&config)?;                                                     // Siehe Output

    // Convenience Wrapper um Module::new, welcher die Möglichkeit gibt eine Datei einzulesen
    // Anschließend wird Module::new aufgerufen, innerhalb der wat::parse_bytes(param: Bytes)
    // aufgerufen wird
    //
    // WENN: bytes.starts_with (b"\0asm\) -> Return OK
    // ELSE: Konvertiert die Bytes zu einem UTF-8 String Slice (str), dabei findet eine Validierung statt
    // ob die vorliegenden Bytes valides UTF-8 sind
    // Anschließend wird der String Slice in geparsed und anschließend in binary Form encodiert
    // Beim Enkodieren kann das Modul dabei modifiziert werden (z.B. werden Shorthands ausgeschrieben
    // oder Module Felder gemischt
    //
    // Da es sich bei einem Modul nur um eine pure Code Representation handelt muss diese
    // instanziiert werden, damit etwa Funktionen aufgerufen werden können.

    let module = Module::from_file(&engine, "./wasm/add.wat")?;

    // Laden eines Moduls aus einem .cwasm File
    let path = Path::new("add.cwasm");
    let buffer = fs::read("./wasm/add.wat")?;
    let precompiled_file = engine.precompile_module(&buffer)?;
    save_precompiled_file(&precompiled_file);
    let pc_module = load_precompiled_file(&engine, path).unwrap();

    // Der Store ist eine Isolationseinheit. Alle Wasm Objekte sind grundsätzlich komplett in einen
    // Store eingebetettet. (z.B. Globals, Tables, Memories, etc.)
    // Bildet einen Kontext für die meisten Wasm Operationen und speichert handles auf Instanzen

    println!("Erstelle Strore ...");
    let mut store = Store::new(&engine, ());
    println!("{:?}", store.data());
    println!("{:?}", store.fuel_consumed());

    let interrupt_handle = store.interrupt_handle();

    // Es könnte sein dass Interrupts für den betreffenden Store nicht aktiviert sind.
    // Im Falle dass sie deatkiviert sind -> Panic
    let interrupt_handle = match interrupt_handle {
        Ok(interrupt_handle) => interrupt_handle,
        Err(error) => panic!("Problem creating InterruptHandle: {}", error)
    };

    store.add_fuel(1000);

    println!("{:?}", interrupt_handle);
    println!("{:?}", store.data());
    println!("{:?}", store.data_mut());

    // Instanziierungsprozess eines Wasm-Moduls
    // Nach Abschluss des Prozesses kann auf die Exporte des entsprechenden Moduls zugegriffen werden
    // Instanzen gehören einem Store an (Wird zur Erstellungszeit als Parameter hineingegeben)
    //
    // Struktur:
    // id: InstanzID
    // exports
    // types
    // signatures
    println!("Instanziieren des Moduls...");
    let instance1 = Instance::new(&mut store, &module, &[])?;
    let instance2 = Instance::new(&mut store, &pc_module, &[])?;
    let instance3 = Instance::new(&mut store, &module, &[])?;
    let instance4 = Instance::new(&mut store, &module, &[])?;

    // get_func durchsucht das Modul nach einer entsprechenden Funktion und gibt sie zurück
    // Wenn die Funktion nicht vorhanden ist wird none zurückgegeben
    // Ein Fehler wird geworfen wenn der Store die entsprechende Instanz nicht owned
    println!("Exportiere Funktion... (add)");
    let add = instance1.get_func(&mut store, "add").expect("Export nicht vorhanden!");
    let add2 = instance2.get_func(&mut store, "add").expect("Export nicht vorhanden");

    println!("Aufrufen der Funktion... (add)");

    let ret = add.call(&mut store, &[wasmtime::Val::I32(2), wasmtime::Val::I32(34)])?;
    println!("Der Rückgabewert der Funktion ist: {:?}\n\n", ret);

    let ret2 = add2.call(&mut store, &[wasmtime::Val::I32(232), wasmtime::Val::I32(3423)])?;
    println!("Der Rückgabewert der Funktion ist: {:?}\n\n", ret2);

    println!("Weitere Informationen \n");
    println!("Default Konfiguration der Engine");
    println!("{:#?}", engine.config());

    println!("{:?}", instance1);
    println!("{:?}", instance2);
    println!("{:?}", instance3);
    println!("{:?}", instance4);
    println!("{:?}",store.data());
    println!("{:?}",store.data_mut());
    Ok(())
}