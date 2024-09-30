<?php
    session_start();

    if(!isset($_SESSION['logged_user'])) {
        echo '
        <html>
            <head>
                <title>404 Not Found</title>
            </head>
            <body>
                <h1>Not Found</h1>
                <p>The requested URL was not found on this server.</p>
            </body>
        </html>';
        exit();
    }

    include "config/config.php"; 
    include_once CLASSES_PATH . "auth.php";
    include_once CLASSES_PATH . "sanitize.php";

    use PhpSanitization\PhpSanitization\Sanitization;

    $auth = new Auth;
    $sanitizer = new Sanitization;

?>

<!DOCTYPE html>
<html>
    <head>
        <title>Halcyon dashboard</title>
        <link rel="stylesheet" href="../css/styles.css">
    </head>

    <body>
        
    </body>

</html>