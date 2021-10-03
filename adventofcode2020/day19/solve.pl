my @rules;
$rules[$1] = $2 while (<> =~ /^(\d+): (.*)$/);

my @regexps;
sub regexp {
    my ($i) = @_;
    $regexps[$i] ||= $rules[$i] =~ /"(.*)"/ ? $1 :
        '(?:' . (join '|', map {join '', map {regexp($_)} split}
                 split / \| /, $rules[$i]) . ')';
}

my $rx = regexp(0);
my @messages = <>;
my $n = grep /^$rx$/, @messages;
print "Part 1: $n\n";

@regexps = (); # clear cache
regexp(42); regexp(31);
$regexps[8] = "$regexps[42]+";
$regexps[11] = "($regexps[42](?1)?$regexps[31])";
$rx = regexp(0);
$n = grep /^$rx$/, @messages;
print "regex: $rx";
print "Part 2: $n";
