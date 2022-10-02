set cell 1 to 56
+++++++[->++++++++<]>

move cell 1 to cell 0
[-<+>]

set cell 0 to 57 (9 in ascii)
<+

move cell 0 to 1 and 2 and 3
[->+>+>+<<<]>>>

move cell 3 to 0
[-<<<+>>>]

set cell 2 to 48 (0 in ascii)
<---------

set cell 6 to 10 (new line in ascii)
>>>>++++++++++

set cell 9 to 99 (for lowercase ascii)
>>>>+++++++++++[-<+++++++++>]

set cell 10 to 80 (for uppercase ascii)
>++++++++++[-<++++++++>]

set cell 11 to 32 (ascii for space)
>++++[-<++++++++>]

set cell 12 to 1
+

set cell 15 to 1
>>>+<<<<

<<<<<
[
    <<<<
    copy cell 2 to 4
    {
        move cell 2 to 4 and 5
        [->>+>+<<<]>>>

        move cell 5 to cell 2
        [-<<<+>>>]<<<
    }

    subtract cell 2 from cell 0
    [-<<->>]<<

    [
        if not 0

        add cell 4 to cell 0 and cell 2
        >>>>[-<<+<<+>>>>]<<<

        copy cell 1 to 3
        {
            move cell 1 to 3 and 4
            [->>+>+<<<]>>>

            move cell 4 to cell 1
            [-<<<+>>>]<<
        }

        copy cell 2 to 4
        {
            move cell 2 to 4 and 5
            [->>+>+<<<]>>>

            move cell 5 to cell 2
            [-<<<+>>>]<<<
        }

        subtract cell 2 from cell 1
        [-<->]<+

        [
            if not 0

            zero out cell 1
            [-]

            move cell 3 to cell 1
            >>[-<<+>>]
        ] >>
        [
            if 0

            move cell 3 to cell 1
            [-<<+>>]

            add 10 to cell 1
            <<++++++++++

            subtract cell 0 by 1
            <-

            go to cell 5
            >>>>>
        ]

        move cell 4 to cell 2
        <[-<<+>>]
    ] >>>>
    [
        if 0

        copy cell 4 to cell 2
        {
            move cell 4 to 2 and 5
            [->+<<<+>>]

            move cell 5 to 4
            >[-<+>]

            go to cell 4
            <
        }

        subtract cell 4 from cell 1 and and move to cell 5
        [-<<<->>>>+<]

        move cell 5 to cell 4
        >[-<+>]<<<<

        [
            if not 0

            -
            [
                if not 0
                >>>>>>
            ] >
            [
                if 0

                set cell 12 to 0
                >>>>>>>>>>-

                go to cell 3
                <<<<
            ]
            <<<<<<<+

            add cell 4 to cell 1 and cell 5
            >>>[->+<<<<+>>>]>

            move cell 5 to cell 4
            [-<+>]

            go to cell 0
            <<<<<
        ] >>>
        [
            if 0
            >>>>>>

            change P to T and back
            ++++.----

            turn c into a
            <--.

            turn a into k
            ++++++++++.

            turn k into e
            ------.

            go to cell 11 and print space
            >>.<<

            turn e into o
            ++++++++++.

            turn o into n
            -.

            turn n into e
            ---------.

            go to cell 11 and print space
            >>.<<

            turn e into d
            -.

            turn d into o
            +++++++++++.

            turn o into w
            ++++++++.

            turn w into n
            ---------.

            go to cell 11 and print space
            >>.<<

            turn n into a
            -------------.

            turn a into n
            +++++++++++++.

            turn n into d
            ----------.

            go to cell 11 and print space
            >>.<<

            turn d into p
            ++++++++++++.

            turn p into a
            ---------------.

            turn a into ss
            ++++++++++++++++++..

            go to cell 11 and print space
            >>.<<

            turn s into i
            ----------.

            turn i into t
            +++++++++++.

            go to cell 11 and print space
            >>.<<

            turn t into a
            -------------------.

            turn a into r
            +++++++++++++++++.

            turn r into o
            ---.

            turn o into u
            ++++++.

            turn u into n
            -------.

            turn n into d
            ----------.

            go to cell 11 and print a comma space
            >>++++++++++++.------------.<<

            turn d into n
            ++++++++++.

            turn n into o
            +.

            go to cell 11 and print space
            >>.<<

            turn o into m
            --.

            turn m into o
            ++.

            turn o into r
            +++.

            turn r into e
            -------------.--

            go to cell 11 and print space
            >>.

            go to cell 9 and print b
            <<-.

            turn b into o
            +++++++++++++.

            turn o into tt
            +++++..

            turn t into l
            --------.

            turn l into e
            -------.

            turn e into s
            ++++++++++++++.

            go to cell 11 and print space
            >>.<<

            turn s into o
            ----.

            turn o into f
            ---------.

            go to cell 11 and print space
            >>.<<

            turn f into b
            ----.

            turn b into ee
            +++..

            turn e into r
            +++++++++++++.

            go to cell 11 and print space
            >>.<<

            turn r into o
            ---.

            turn o into n
            -.

            go to cell 11 and print space
            >>.<<

            turn n into t
            ++++++.

            turn t into h
            ------------.

            turn h into e
            ---.

            go to cell 11 and print space
            >>.<<

            turn e into w
            ++++++++++++++++++.

            turn w into a
            ----------------------.

            turn a into ll
            +++++++++++..+++

            go to cell 11 and print a period
            >>++++++++++++++.--------------<<

            print 2 new lines
            <<<..

            >>>>--.<.

            go to cell 11 and print space
            >>.<<

            turn o into m
            --.

            turn m into o
            ++.

            turn o into r
            +++.

            turn r into e
            -------------.

            go to cell 11 and print space
            >>.

            go to cell 9 and print b
            <<---.

            turn b into o
            +++++++++++++.

            turn o into tt
            +++++..

            turn t into l
            --------.

            turn l into e
            -------.

            turn e into s
            ++++++++++++++.

            go to cell 11 and print space
            >>.<<

            turn s into o
            ----.

            turn o into f
            ---------.

            go to cell 11 and print space
            >>.<<

            turn f into b
            ----.

            turn b into ee
            +++..

            turn e into r
            +++++++++++++.

            go to cell 11 and print space
            >>.<<

            turn r into o
            ---.

            turn o into n
            -.

            go to cell 11 and print space
            >>.<<

            turn n into t
            ++++++.

            turn t into h
            ------------.

            turn h into e
            ---.

            go to cell 11 and print space
            >>.<<

            turn e into w
            ++++++++++++++++++.

            turn w into a
            ----------------------.

            turn a into ll
            +++++++++++..

            go to cell 11 and print comma space
            >>++++++++++++.------------.<<

            turn l into n
            ++.

            turn n into o
            +.

            go to cell 11 and print space
            >>.<<

            turn o into m
            --.

            turn m into o
            ++.

            turn o into r
            +++.

            turn r into e
            -------------.--

            go to cell 11 and print space
            >>.

            go to cell 9 and print b
            <<-.

            turn b into o
            +++++++++++++.

            turn o into tt
            +++++..

            turn t into l
            --------.

            turn l into e
            -------.

            turn e into s
            ++++++++++++++.

            go to cell 11 and print space
            >>.<<

            turn s into 0
            ----.

            turn o into f
            ---------.

            go to cell 11 and print space
            >>.<<

            turn f into b
            ----.

            turn b into ee
            +++..

            turn e into r
            +++++++++++++.

            go to cell 11 and print a period
            >>++++++++++++++.--------------<<

            go to cell 6 and print new line
            <<<.>>>>

            print G
            -------.<

            turn r into o
            ---.

            go to cell 11 and print space
            >>.<<

            turn o into t
            +++++.

            turn t to o
            -----.

            go to cell 11 and print space
            >>.<<

            turn o to t
            +++++.

            turn t to h
            ------------.

            turn h to e
            ---.

            go to cell 11 and print space
            >>.<<

            turn e to s
            ++++++++++++++.

            turn s to t
            +.

            turn t to o
            -----.

            turn o to r
            +++.

            turn r to e
            -------------.

            go to cell 11 and print space
            >>.<<

            turn e to a
            ----.

            turn a to n
            +++++++++++++.

            turn n to d
            ----------.

            go to cell 11 and print space
            >>.<<

            turn d to b
            --.

            turn b to u
            +++++++++++++++++++.

            turn u to y
            ++++.

            go to cell 11 and print space
            >>.<<

            turn y to s
            ------.

            turn s to o
            ----.

            turn o to m
            --.

            turn m to e
            --------.

            go to cell 11 and print space
            >>.<<

            turn e to m
            ++++++++.

            turn m to o
            ++.

            turn o to r
            +++.

            turn r to e
            -------------.

            go to cell 11 and print comma space
            >>++++++++++++.------------.<<

            <<<<<+++++++++..>>>>>

            go to cell 11 and print space
            >>.

            go to cell 9 and print b
            <<---.

            turn b into o
            +++++++++++++.

            turn o into tt
            +++++..

            turn t into l
            --------.

            turn l into e
            -------.

            turn e into s
            ++++++++++++++.

            go to cell 11 and print space
            >>.<<

            turn s into o
            ----.

            turn o into f
            ---------.

            go to cell 11 and print space
            >>.<<

            turn f into b
            ----.

            turn b into ee
            +++..

            turn e into r
            +++++++++++++.

            go to cell 11 and print space
            >>.<<

            turn r into o
            ---.

            turn o into n
            -.

            go to cell 11 and print space
            >>.<<

            turn n into t
            ++++++.

            turn t into h
            ------------.

            turn h into e
            ---.

            go to cell 11 and print space
            >>.<<

            turn e into w
            ++++++++++++++++++.

            turn w into a
            ----------------------.

            turn a into ll
            +++++++++++..---------

            go to cell 11 and print comma space
            >>++++++++++++++.>
        ]

        add cell 4 to cell 0
        >[-<<<<+>>>>]

        go to cell 8
        >>>>
    ]

    >
    go to cell 9 and print bottles of beer on the wall
    [
        >>>>>>
        [
            if not 0
            [-]
            <<
        ]<<<<<
        [
            if 0

            change P to T and back
            ++++.----

            turn c into a
            <--.

            turn a into k
            ++++++++++.

            turn k into e
            ------.

            go to cell 11 and print space
            >>.<<

            turn e into o
            ++++++++++.

            turn o into n
            -.

            turn n into e
            ---------.

            go to cell 11 and print space
            >>.<<

            turn e into d
            -.

            turn d into o
            +++++++++++.

            turn o into w
            ++++++++.

            turn w into n
            ---------.

            go to cell 11 and print space
            >>.<<

            turn n into a
            -------------.

            turn a into n
            +++++++++++++.

            turn n into d
            ----------.

            go to cell 11 and print space
            >>.<<

            turn d into p
            ++++++++++++.

            turn p into a
            ---------------.

            turn a into ss
            ++++++++++++++++++..

            go to cell 11 and print space
            >>.<<

            turn s into i
            ----------.

            turn i into t
            +++++++++++.

            go to cell 11 and print space
            >>.<<

            turn t into a
            -------------------.

            turn a into r
            +++++++++++++++++.

            turn r into o
            ---.

            turn o into u
            ++++++.

            turn u into n
            -------.

            turn n into d
            ----------.-

            go to cell 11 and print a comma space
            >>++++++++++++.------------.<<

            go to cell 2
            <<<<<<<

            move cell 2 to 4 and 5
            [->>+>+<<<]>>>

            subtract cell 4 from cell 0
            <[-<<<<->>>>]<<<<

            [
                if not 0

                add cell 5 to cell 0 and move to cell 2
                >>>>>[-<<<+<<+>>>>>]<<<<<

                print cell 0 and cell 1
                .>.

                go to cell 3
                >>
            ] >
            [
                if 0

                add cell 5 to cell 0 and move to cell 2
                >>>>[-<<<+<<+>>>>>]<<<<

                print cell 1
                .>>>
            ]

            go to cell 9
            >>>>>

            go to cell 11 and print space
            >>.

            go to cell 9 and print b
            <<-.

            turn b into o
            +++++++++++++.

            turn o into tt
            +++++..

            turn t into l
            --------.

            turn l into e
            -------.

            turn e into s
            ++++++++++++++

            >>>
            [
                if not 0
                <<<.>>>
                >>
            ] <<<<<<
            [
                >>
            ]
            >

            go to cell 11 and print space
            >>.<<

            turn s into o
            ----.

            turn o into f
            ---------.

            go to cell 11 and print space
            >>.<<

            turn f into b
            ----.

            turn b into ee
            +++..

            turn e into r
            +++++++++++++.

            go to cell 11 and print space
            >>.<<

            turn r into o
            ---.

            turn o into n
            -.

            go to cell 11 and print space
            >>.<<

            turn n into t
            ++++++.

            turn t into h
            ------------.

            turn h into e
            ---.

            go to cell 11 and print space
            >>.<<

            turn e into w
            ++++++++++++++++++.

            turn w into a
            ----------------------.

            turn a into ll
            +++++++++++..---------

            go to cell 11 and print a period
            >>++++++++++++++.--------------<<

            print 2 new lines
            <<<..

            go to cell 8
            >>
        ]

        go to cell 2
        <<<<<<

        move cell 2 to 4 and 5
        [->>+>+<<<]>>>

        subtract cell 4 from cell 0
        <[-<<<<->>>>]<<<<

        [
            if not 0

            add cell 5 to cell 0 and move to cell 2
            >>>>>[-<<<+<<+>>>>>]<<<<<

            print cell 0 and cell 1
            .>.

            go to cell 3
            >>
        ] >
        [
            if 0

            add cell 5 to cell 0 and move to cell 2
            >>>>[-<<<+<<+>>>>>]<<<<

            print cell 1
            .>>>
        ]

        go to cell 9
        >>>>>

        go to cell 11 and print space
        >>.

        go to cell 9 and print b
        <<-.

        turn b into o
        +++++++++++++.

        turn o into tt
        +++++..

        turn t into l
        --------.

        turn l into e
        -------.

        turn e into s
        ++++++++++++++

        >>>
        [
            if not 0
            <<<.>>>
            >>
        ] <<<<<<
        [
            >>
        ]
        >

        go to cell 11 and print space
        >>.<<

        turn s into o
        ----.

        turn o into f
        ---------.

        go to cell 11 and print space
        >>.<<

        turn f into b
        ----.

        turn b into ee
        +++..

        turn e into r
        +++++++++++++.

        go to cell 11 and print space
        >>.<<

        turn r into o
        ---.

        turn o into n
        -.

        go to cell 11 and print space
        >>.<<

        turn n into t
        ++++++.

        turn t into h
        ------------.

        turn h into e
        ---.

        go to cell 11 and print space
        >>.<<

        turn e into w
        ++++++++++++++++++.

        turn w into a
        ----------------------.

        turn a into ll
        +++++++++++..---------

        go to cell 11 and print comma space
        >>++++++++++++.------------.<<

        go to cell 2
        <<<<<<<

        move cell 2 to 4 and 5
        [->>+>+<<<]>>>

        subtract cell 4 from cell 0
        <[-<<<<->>>>]<<<<

        [
            if not 0

            add cell 5 to cell 0 and move to cell 2
            >>>>>[-<<<+<<+>>>>>]<<<<<

            print cell 0 and cell 1
            .>.

            go to cell 3
            >>
        ] >
        [
            if 0

            add cell 5 to cell 0 and move to cell 2
            >>>>[-<<<+<<+>>>>>]<<<<

            print cell 1
            .>>>
        ]

        go to cell 9
        >>>>>

        go to cell 11 and print space
        >>.

        go to cell 9 and print b
        <<-.

        turn b into o
        +++++++++++++.

        turn o into tt
        +++++..

        turn t into l
        --------.

        turn l into e
        -------.

        turn e into s
        ++++++++++++++

        >>>
        [
            if not 0
            <<<.>>>
            >>
        ] <<<<<<
        [
            >>
        ]
        >

        go to cell 11 and print space
        >>.<<

        turn s into 0
        ----.

        turn o into f
        ---------.

        go to cell 11 and print space
        >>.<<

        turn f into b
        ----.

        turn b into ee
        +++..

        turn e into r
        +++++++++++++.---------------

        go to cell 11 and print period
        >>++++++++++++++.--------------<<

        go to cell 6 and print new line
        <<<.<
    ]

    move back to cell 1 and subtract it
    <<<<->>>>>
]