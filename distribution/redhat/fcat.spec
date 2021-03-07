%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: fcat
Summary: Flatten nested file formats like json, toml, yaml into single lines with full path to all values.
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: MIT or ASL 2.0
Group: Applications/System
Source0: %{name}-%{version}.tar.gz
URL: https://fcat.pustina.de

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
fcat
 - is very fast

%prep
%setup -q

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
