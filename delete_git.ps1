$bash_path = "D:\Learning\codes\Rust\luogu";

$all_child = Get-ChildItem -Path $bash_path;

foreach ($child in $all_child) {
    Write-Host "Checking " $child.Name " ...";
    if (Test-Path -Path $child.FullName -PathType Container) {
        foreach ($sub_child in Get-ChildItem -Path $child -Hidden) {
            Write-Host "Checking " $sub_child.NameString " ...";
            if ($sub_child.NameString -like "*.git*") {
                Write-Host "Delete " $sub_child.Name " .git";
                remove-item -path $sub_child -recurse -force;
            }
        }
    }
}