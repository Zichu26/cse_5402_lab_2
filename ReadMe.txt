Lab number: CSE 5402 Fall 2025 Lab 2
Zichu Pan p.zichu@wustl.edu
Edgar Palomino e.j.palomino@wustl.edu

Structs:
    Player Struct:
        Data: 
            character name, 
            lines (as tuples of line number and text)
            current line index

        Methods:
            new(): Constructor that creates a new player with a given name
            prepare(): Reads a part file, parses line numbers, and sorts lines
                add_script_line(): Private helper that parses individual lines
            speak(): Outputs the next line and updates the current speaker
            next_line(): Returns the line number of the next unspoken line (or None)
    
        Play Struct:
            Data: 
                play title
                vector of Player structs

            Methods:
                new(): Constructor that creates an empty play
                prepare(): Main entry point that reads config file and initializes all players
                read_config(): Parses the configuration file and extracts title and character info
                    dd_config(): Private helper that parses individual config lines
                process_config(): Creates and prepares Player instances from config data
                recite(): Orchestrates the performance by having players speak in line number order
    
    Design Challenges:
        The play needs to deliver lines in global line number order, but each player only knows their own lines. To solve this
        the recite() method iterates through all players each turn, finds the player with the smallest next line number using 
        next_line(), and has that player speak(). This continues until all players have exhausted their lines.

        The program also needs to detect and warn about missing or duplicate line numbers in whinge mode. To solve this
        the recite() method maintains an expected_line_number counter that tracks what line should come next. By comparing 
        each actual line number (note: actual line number is always the smallest line number from all possible players) to the 
        expected value: 
            If actual > expected: missing lines detected (warn about each gap)
            If actual == expected: normal case (increment expected)
            If actual < expected: duplicate detected (we've seen this number before)
    
Return Wrapper:
    Data:
        A single code: u8 field that stores the exit code value

    The Termination trait implementation handles the conversion from our custom type to a shell exit code. The report() method:
        1. Checks if the code is non-zero (indicating an error)
        2. If non-zero, prints a formatted error message to stderr: "Error: {code}"
        3. Converts the u8 code to an ExitCode using ExitCode::from()
        4. Returns the ExitCode for the shell environment

    The main function was modified to change return type from Result<(), u8> to ReturnWrapper and wrap all return values 
    with ReturnWrapper::new():
        ReturnWrapper::new(0) for successful execution
        ReturnWrapper::new(BAD_COMMAND_LINE_ERROR) for command-line argument errors
        ReturnWrapper::new(SCRIPT_GENERATION_ERROR) for file I/O or parsing errors
    
    No design Challenges encountered.



