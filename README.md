# diagnostick
A simple tool to check the configuration and the state of your computer *(linux only)* 

## Run
Use ```cargo``` to run diagnostick:

```bash
cargo run
```

## Options

**Getting options**

Use ```cargo``` to get options and bring the documentation:
```bash
cargo run -- --help
```

**Loading configuration file**

Use ```cargo``` to load a configuration file:
```bash
cargo run -- --configuration-file <PATH>
```

## Output
For the moment, the output is in the file ```output.json``` :
```JSON
{
    "key": "value"
}
```
