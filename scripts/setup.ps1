$rdd = New-Object System.Management.Automation.Host.ChoiceDescription '&1) Run Desktop Debug'
$rdr = New-Object System.Management.Automation.Host.ChoiceDescription '&2) Run Desktop Release', 'rdr'
$bdd = New-Object System.Management.Automation.Host.ChoiceDescription '&3) Bundle Desktop Debug'
$bdr = New-Object System.Management.Automation.Host.ChoiceDescription '&4) Bundle Desktop Release'

$rwd = New-Object System.Management.Automation.Host.ChoiceDescription '&5) Run Web Debug'
$rwr = New-Object System.Management.Automation.Host.ChoiceDescription '&6) Run Web Release'
$bwd = New-Object System.Management.Automation.Host.ChoiceDescription '&7) Build Web Debug'
$bwr = New-Object System.Management.Automation.Host.ChoiceDescription '&8) Build Web Release'

$gtc = New-Object System.Management.Automation.Host.ChoiceDescription '&9) Generate Tailwind CSS'

$quit = New-Object System.Management.Automation.Host.ChoiceDescription '&Q) Quit'


$options = [System.Management.Automation.Host.ChoiceDescription[]]($rdd, $rdr, $bdd, $bdr, $rwd, $rwr, $bwd, $bwr, $gtc, $quit)


$message = 'Select an option:'
$choice = $host.ui.PromptForChoice($title, $message, $options, 0)

Switch ($choice)
{
    0 {
        Write-Host "dx serve --hot-reload --platform desktop"
        dx serve --hot-reload --platform desktop
    }
    1 {
        Write-Host "dx serve --hot-reload --platform desktop --release"
        dx serve --hot-reload --platform desktop --release
    }
    2 {
        Write-Host "dx bundle --platform desktop"
        dx bundle --platform desktop
    }
    3 {
        Write-Host "dx bundle --platform desktop --release"
        dx bundle --platform desktop --release
    }

    4 {
        Write-Host "dx serve --hot-reload --platform web"
        dx serve --hot-reload --platform web
    }
    5 {
        Write-Host "dx serve --hot-reload --platform web --release"
        dx serve --hot-reload --platform web --release
    }
    6 {
        Write-Host "dx build --platform web"
        dx build --platform web
    }
    7 {
        Write-Host "dx build --platform web --release"
        dx build --platform web --release
    }

    8 {
        Write-Host "npx tailwindcss --config ./tailwind.config.js -i ./input.css -o ./assets/tailwind.css --minify --watch"
        npx tailwindcss build -o src/tailwind.css # TODO: Update command
    }

    9 {
        Write-Host "Goodbye!"
        break
    }
}