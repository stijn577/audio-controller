def "gg build" [] {
    print "Building all..."
    gg build client
    gg build server
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
    cargo clean
    cd client
    cargo clean
    cd ../server
    cargo clean
    
}