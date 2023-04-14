$bash_path = "D:\Learning\codes\Rust\luogu";

$all_child = Get-ChildItem -Path $bash_path;

foreach ($child in $all_child) {
    if (Test-Path -Path $child.FullName -PathType Container) {
        $tmp = Get-ChildItem -Path $child;
        foreach ($sub_child in $tmp) {
            Write-Host "Checking " $sub_child.FullName " ...";
            if ($sub_child.NameString -like "*.git*") {
                Write-Host "Delete " $sub_child.FullName;
                remove-item -path $sub_child -recurse -force;
            }

            if ($sub_child.NameString -like "*.gitignore*") {
                Write-Host "Delete " $sub_child.Name " .git";
                remove-item -path $sub_child -recurse -force;
            }
        }
    }
}