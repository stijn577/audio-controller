def "gg build" [] {
    print "Building all..."
    gg build client
    gg build server
}

def "gg build client" [] {
    print "Building client firmware..."
    cd ~/Documents/projects/audio-controller/software/client
    cargo build -r -q
}

def "gg build server" [] {
    print "Building server software..."
    cd ~/Documents/projects/audio-controller/software
    cargo build -q --package server 
}

def "gg client" [] {
    print "Running client..."
    cd ~/Documents/projects/audio-controller/software/client
    cargo r -r -q
}

def "gg server" [] {
    print "Running server..."
    cd ~/Documents/projects/audio-controller/software
    cargo r -q --package server 
}

