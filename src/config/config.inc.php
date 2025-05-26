<?php
declare(strict_types=1);

$cfg['blowfish_secret'] = 'l6K#.xbybx6s/AyjgQUTUX-:)@DP<,q7';

$i = 0;
$i++;

$cfg['Servers'][$i]['auth_type'] = 'config';
$cfg['Servers'][$i]['host'] = '127.0.0.1';
$cfg['Servers'][$i]['port'] = '3306';
$cfg['Servers'][$i]['user'] = 'root';
$cfg['Servers'][$i]['password'] = '';
$cfg['Servers'][$i]['compress'] = false;
$cfg['Servers'][$i]['AllowNoPassword'] = true;

$cfg['ForceSSL'] = false; 
$cfg['AllowUserDropDatabase'] = false;
$cfg['ShowDatabasesNavigationAsTree'] = false;
// $cfg['LoginCookieValidity'] = 14400;

$cfg['AllowArbitraryServer'] = false;
$cfg['DefaultLang'] = 'en';
$cfg['DefaultDisplay'] = 'horizontal';
$cfg['RepeatCells'] = 100;
$cfg['MaxRows'] = 25;
$cfg['RowActionLinks'] = 'left';
$cfg['DisableMultiTableMaintenance'] = false;
$cfg['DefaultTabServer'] = 'main.php';
$cfg['DefaultTabDatabase'] = 'db_structure.php';
$cfg['DefaultTabTable'] = 'tbl_structure.php';

$cfg['ThemeDefault'] = 'pmahomme';

$cfg['SendErrorReports'] = 'never';
$cfg['ConsoleEnterExecutes'] = false;