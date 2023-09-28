#!perl
use Mojolicious::Lite -signatures;
use Mojo::JSON qw/decode_json encode_json/;
use Carp;
use lib './lib';
use FighterGame::L10N;
use Getopt::Long;
use Pod::Usage;

# Argument stuff
# TODO: make it actually affect the program
my $game_prefix = '';
my $lang = 'en-us';
GetOptions(
    's|prefix=s' => \$game_prefix,
    'h|help'     => \my $help,
    'p|port'     => \my $port,
    'a|address'  => \my $addr,
    'l|lang'     => \$lang,
) or pod2usage(2);
pod2usage(1) if defined $help;

# Setup localization
my $loc = FighterGame::L10N->get_handle($lang);
# Declare underscore function
sub _ {$loc->maketext(@_)}
# Only allow templates from the directory in resources
@{app->renderer->paths} = ('../resources/templates');
@{app->static->paths} = ('../resources/static');
# Web socket for communication
websocket "$game_prefix/game" => sub($c) {
    $c->on(json => sub ($c0, $hash) {
        say encode_json $hash;
    });
};

# The game view itself
get "$game_prefix/" =>
    {
        title => _('Fighter'),
        lang  => $loc->language_tag,
    } => 'index';

app->start;

__END__
=head1 NAME

server.pl - Server for FighterGame

=head1 SYNOPSIS

server.pl [options]

=head1 OPTIONS

=over 4

=item C<-s, --prefix>

the prefix to the url for the game stuff

=item C<-h, --help>

show this help

=item C<-p, --port>

set the port for the server

=item C<-a, --address>

set the address for the server

=item C<-l, --lang>

set the server language

=back

=head1 COPYRIGHT AND LICENSE

This program is licensed under the same terms as Perl.

