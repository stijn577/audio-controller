

def "gg server" [] {
    cd software/server
    cargo r 
}

def "gg client" [] {
    
}

def "gg clean all" [] {
    cd software
    cargo clean
    cd client
    cargo clean
    cd ../server
    cargo clean
    
}