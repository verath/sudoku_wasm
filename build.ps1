$GIT_VERSION = . {
    git diff-index --quiet HEAD
    if (-not $?) {
        Write-Warning 'working directory has uncommited changes'
    }

    $s = git rev-parse --short HEAD
    if (-not $?) {
        throw 'error calling git rev-parse'
    }
    return $s
}

# Clear old dist
Remove-Item -R .\dist\* -ErrorAction SilentlyContinue
New-Item -ItemType Directory .\dist -ErrorAction SilentlyContinue

# Generate solver wasm
wasm-pack build --target web solver-wasm

# Copy wasm output
Copy-Item .\solver-wasm\pkg\solver_wasm.js .\dist\
Copy-Item .\solver-wasm\pkg\solver_wasm_bg.wasm .\dist\

# Copy static web
Copy-Item .\web\index.html .\dist\
Copy-Item .\web\main.css .\dist\
Copy-Item .\web\spinner.css .\dist\

# Set GIT_VERSION in index.html
$c = Get-Content .\dist\index.html
Set-Content .\dist\index.html -Value $c.Replace('@GIT_VERSION@', $GIT_VERSION)
