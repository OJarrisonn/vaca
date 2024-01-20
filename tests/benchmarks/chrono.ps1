param(
    [Parameter(Mandatory=$true)]
    [string]$Command,

    [Parameter(Mandatory=$true)]
    [int]$Times
)

$total = 0

for ($i=0; $i -lt $Times; $i++) {
    $start = Get-Date
    Invoke-Expression $Command > $null
    $end = Get-Date
    $total += ($end - $start).TotalMilliseconds
}

$average = $total / $Times
Write-Host "Average execution time: $average ms"