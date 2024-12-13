use strict;
use warnings;

my $input = do { local $/; <STDIN>};
my $re = qr/
    (?<mul_op>mul\(        # literal match of mul(
        (?<lhs>\d{1,3})    # Capture the inner digit that can be 1-3 digits long
        ,                  # literal literal comma
        (?<rhs>\d{1,3})\)  # Capture the inner digit that can be 1-3 digits long
    )|
    (?<do>do\(\))|         # Can match a do() literal
    (?<dont>don\'t\(\))    # Can match a don't() literal
/x;

my $execute = 1;
my $total = 0;
while ($input =~ /$re/g) {
    if (exists($+{do})) {
        $execute = 1;
    } elsif (exists($+{dont})) {
        $execute = 0;
    } elsif (exists($+{mul_op}) && $execute) {
        $total = $total + ($+{lhs} * $+{rhs});
    }
}
print "Total: $total\n";
