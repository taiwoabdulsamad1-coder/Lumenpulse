Param(
    [string]$Network = "futurenet",
    [string]$SourceKeyName = "lumenpulse-admin"
)

Write-Host "Building Soroban contract (lumenpulse-contract) for wasm..."
cargo build -p lumenpulse-contract --target wasm32-unknown-unknown --release

$wasmPath = "..\..\on-chain\target\wasm32-unknown-unknown\release\lumenpulse_contract.wasm"
if (-Not (Test-Path $wasmPath)) {
    $wasmPath = "target\wasm32-unknown-unknown\release\lumenpulse_contract.wasm"
}

if (-Not (Test-Path $wasmPath)) {
    Write-Error "WASM artifact not found at: $wasmPath"
    exit 1
}

Write-Host "Ensuring Soroban key exists: $SourceKeyName"
soroban keys generate $SourceKeyName

Write-Host "Deploying contract to $Network..."
$deploy = soroban contract deploy --wasm $wasmPath --source $SourceKeyName --network $Network
Write-Host "Deploy output:" $deploy

Write-Host "Done. Use the returned contract ID to invoke methods."