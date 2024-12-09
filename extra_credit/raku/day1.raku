

sub MAIN(Str :$input_file) {
    my $sides = reduce {$^a[0].push($^b[0]); $^a[1].push($^b[1]); $^a}, ([],[]), |$input_file.IO.lines.map({$_.split('  ').map({Int($_)})}).map({$_.sort});
    say ($sides[0] Z $sides[1]).map({($_[0] - $_[1]).abs}).sum;
}