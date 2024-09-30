<?php
    include_once("database.php");
    include_once("sanitize.php");

    use PhpSanitization\PhpSanitization\Sanitization;

    class Util {
        private $db;
        private $sanitize;

        public function __construct() {
            $this->db = new Database;
            $this->sanitize = new Sanitization;
        }

        public function sanitizeInput($input) {
            return $this->sanitize->useSanitize($input);
        }

        public function getRequest($url) {
            $ch = curl_init($url);
            curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
            curl_setopt($ch, CURLOPT_HEADER, 0);
            return curl_exec($ch);
        }

        public function postRequest($url, $data) {
            $ch = curl_init($url);
            curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
            curl_setopt($ch, CURLOPT_HEADER, 0);
            curl_setopt($ch, CURLOPT_POST, 1);
            curl_setopt($ch, CURLOPT_POSTFIELDS, $data);
        
            return curl_exec($ch);
        }
    }