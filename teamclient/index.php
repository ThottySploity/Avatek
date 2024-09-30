<?php
    session_start();
    session_destroy();
    session_start();

    include "php/config/config.php";
    include_once CLASSES_PATH . "auth.php";
    include_once CLASSES_PATH . "utilities.php";

    $auth = new Auth;
    $util = new Util;

    if ($_SERVER['REQUEST_METHOD'] === "POST") {
        if(isset($_POST['username']) && isset($_POST['password'])) {
            $username = $util->sanitizeInput($_POST['username']);
            $password = $util->sanitizeInput($_POST['password']);

            if($auth->checkLogon($username, $password)) {
                session_regenerate_id();
                $_SESSION['logged_user'] = $username;
                header("location: php/panel.php");
            }
        }
    }

?>

<!DOCTYPE html>
<html>
<head>
	<title>Login Page</title>
    <link rel="stylesheet" type="text/css" href="css/styles.css">
    <link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.1.3/css/bootstrap.min.css" integrity="sha384-MCw98/SFnGE8fJT3GXwEOngsV7Zt27NXFoaoApmYm81iuXoPkFOJwJ8ERdknLPMO" crossorigin="anonymous">
</head>
    <body class="main-bg">
        <div class="login-container text-c animated flipInX">
            <div>
                <h1 class="logo-badge text-whitesmoke"><span class="fa fa-user-circle"></span></h1>
            </div>
            <div class="container-content">
                <form action="" method="post">
                    <div class="form-group">
                        <input id="username" name="username" required type="text" class="form-control" placeholder="Username" required="">
                    </div>
                    <div class="form-group">
                        <input id="password" name="password" required type="password" class="form-control" placeholder="*****" required="">
                    </div>
                    <button type="submit" class="form-button button-l margin-b">Sign In</button>
                </form>
                <p class="margin-t text-whitesmoke"><small> Halcyon &copy; 2023</small> </p>
            </div>
        </div>
    </body>
</html>