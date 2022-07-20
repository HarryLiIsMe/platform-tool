@REM test get balance
cargo run  -- account getbalance -u http://8.210.218.74:8545 0x13F052C8A68eB14565c5110f48DBA184B3706591 


@REM test deploy
cargo run  -- contract deploy -u http://8.210.218.74:8545   -a .\src\example\test.abi -c .\src\example\test.bin -k a775466a172919f30e754558ae2f5c74d5d3f28f0c7a18198739c96ce73cc50d


@REM test call
cargo run  -- contract call -u http://8.210.218.74:8545  -t 0x34c2fe8d6965c70bfb980afb8b8e0fbeede5f55f -a .\src\example\test.abi -k a775466a172919f30e754558ae2f5c74d5d3f28f0c7a18198739c96ce73cc50d


@REM test query
cargo run  -- contract query -u http://8.210.218.74:8545  -t 0x34c2fe8d6965c70bfb980afb8b8e0fbeede5f55f -a .\src\example\test.abi 

