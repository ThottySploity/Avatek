<?php
    require_once __DIR__."../../config/config.php";

    class Database {
        private $host = DB_HOST;
        private $user = DB_USER;
        private $pass = DB_PASS;
        private $dbname = DB_NAME;

        private $dbh;
        private $error;
        private $stmt;

        public function __construct() {
            //data source name
            $dsn = 'mysql:host=' . $this->host . ';dbname=' . $this->dbname;
            $options = array(
                PDO::ATTR_ERRMODE => PDO::ERRMODE_EXCEPTION,
            );

            // Create a new PDO instanace
            try {
                $this->dbh = new PDO($dsn, $this->user, $this->pass, $options);

            } // Catch any errors
            catch (PDOException $e) {
                $this->error = $e->getMessage() . PHP_EOL;
            }
        }
        
        public function getError() {
            return $this->error;
        }
    
        public function query($query) {
            $this->stmt = $this->dbh->prepare($query);
        }

        public function execute() {
            return $this->stmt->execute();
        }

        public function resultset() {
            return $this->stmt->fetchAll(PDO::FETCH_OBJ);
        }

        public function single() {
        return $this->stmt->fetch(PDO::FETCH_OBJ);
    }

        public function bind($param, $value, $type = null) {
            if (is_null($type)) {
                switch (true) {
                    case is_int($value):
                        $type = PDO::PARAM_INT;
                        break;
                    case is_bool($value):
                        $type = PDO::PARAM_BOOL;
                        break;
                    case is_null($value):
                        $type = PDO::PARAM_NULL;
                        break;
                    default:
                        $type = PDO::PARAM_STR;
                }
            }
            $this->stmt->bindValue($param, $value, $type);
        }
    }
?>
