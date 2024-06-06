def "gg build" [] {
    print "Building all..."
    gg build macros
    gg build shared
    gg build client
    gg build server
}

def "gg build macros" [] {
    print "Building macros..."
    cd macros
    cargo build -q
}

def "gg build shared" [] {
    print "Building shared data..."
    cargo build -q
}

def "gg build client" [] {
    print "Building client firmware..."
    cd client
    cargo build -q
}

def "gg build server" [] {
    print "Building server software..."
    cd server
    cargo build -q
}

def "gg client" [] {
    cd client
    cargo r
}

def "gg server" [] {
    cd server
    cargo r
}


def "gg clean" [] {
    cd software
    cargo clean
    cd client
    cargo clean
    cd ../server
    cargo clean
    
}