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
    print "Running client..."
    cargo r -r -q
}

def "gg server" [] {
    print "Running server..."
    cargo r -q --package server 
}

