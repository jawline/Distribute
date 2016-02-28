'use strict';

/* App Module */

var imcluelessSite = angular.module('imcluelessSite', [ 'ngRoute' ]);

imcluelessSite.config([ '$routeProvider', function($routeProvider) {
	$routeProvider.when('/cryptography', {
		templateUrl : 'partials/crypto.html'
	}).when('/compsec', {
		templateUrl : 'partials/compsec.html'
	}).when('/crime', {
		templateUrl : 'partials/crime.html'
	}).when('/netsec', {
		templateUrl : 'partials/netsec.html'
	}).when('/soceng', {
		templateUrl : 'partials/soceng.html'
	}).when('/policy', {
		templateUrl : 'partials/policy.html'
	}).when('/integrity', {
		templateUrl : 'partials/integrity.html'
	}).when('/cysec', {
		templateUrl : 'partials/cysec.html'
	}).when('/auth', {
		templateUrl : 'partials/auth.html'
	}).when('/analysis', {
		templateUrl : 'partials/analysis.html'
	}).when('/databases', {
		templateUrl : 'partials/databases.html'
	}).when('/reveng', {
		templateUrl : 'partials/reveng.html'
	}).when('/acl', {
		templateUrl : 'partials/acl.html'
	}).when('/keys', {
		templateUrl : 'partials/keys.html'
	}).when('/confidentiality', {
		templateUrl : 'partials/confidentiality.html'
	}).when('/privacy', {
		templateUrl : 'partials/privacy.html'
	}).when('/standards', {
		templateUrl : 'partials/standards.html'
	}).when('/polob', {
		templateUrl : 'partials/polob.html'
	}).when('/secstrat', {
		templateUrl : 'partials/secstrat.html'
	}).when('/war', {
		templateUrl : 'partials/war.html'
	}).when('/malware', {
		templateUrl : 'partials/malware.html'
	}).when('/media', {
		templateUrl : 'partials/media.html'
	}).when('/ddos', {
		templateUrl : 'partials/ddos.html'
	}).when('/firewall', {
		templateUrl : 'partials/firewall.html'
	}).when('/datac', {
		templateUrl : 'partials/datac.html'
	}).when('/error404', {
		templateUrl : 'partials/error404.html'
	}).otherwise({
		redirectTo : '/error404'
	});
}]).run(function ($rootScope) {});
