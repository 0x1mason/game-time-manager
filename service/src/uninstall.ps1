# snippet from https://serverfault.com/a/1058407/71017
if (!
    (New-Object Security.Principal.WindowsPrincipal(
        [Security.Principal.WindowsIdentity]::GetCurrent()
    )).IsInRole(
        [Security.Principal.WindowsBuiltInRole]::Administrator
    )
) {
    Start-Process `
        -FilePath 'powershell' `
        -ArgumentList (
        #flatten to single array
        '-File', $MyInvocation.MyCommand.Source, $args `
        | % { $_ }
    ) `
        -Verb RunAs
    exit
}

Remove-Item "$Env:USERPROFILE\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\GameTimeManager.lnk"

# get process
$proc = Get-Process GameTimeManager -ErrorAction SilentlyContinue
if ($proc) {
    Stop-Process -Name GameTimeManager

    # should have loop with timeout full sleep
    Start-Sleep 3
    
    if (!$proc.HasExited) {
        Stop-Process -Force -Name GameTimeManager
    }
}

Write-Output "Successfully uninstalled Game Time Monitor."

Pause