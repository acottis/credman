# user: &str, pass: &str, service: &str
$source = @"
[DllImport("./target/debug/credman.dll", CharSet = CharSet.Ansi)]
public static extern void store(string user, string pass, string service);

[DllImport("./target/debug/credman.dll", CharSet = CharSet.Ansi)]
public static extern void read(string service);

[DllImport("./target/debug/credman.dll")]
public static extern uint add(uint a, uint b);
"@


$rust = Add-Type -MemberDefinition $source -Name 'CredMan' -Namespace 'Azphel' -PassThru

$rust::store("Adam", "Testing", "Azphel:CredMan")
$rust::read("Azphel:CredMan")