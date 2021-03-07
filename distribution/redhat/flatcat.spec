%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: flatcat
Summary: Flatten nested file formats like JSON, TOML, YAML into single lines with full path to all values.
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: MIT or ASL 2.0
Group: Applications/System
Source0: %{name}-%{version}.tar.gz
URL: https://flatcat.pustina.de

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
flatcat
Flatten nested file formats like JSON, TOML, YAML into single lines with full path to all values.
- Supports JSON, TOML, YAML, and more format are coming.
- Support colorful output to ease readability
- Allows to ignore `Null` values
- Unrecognized file formats are printed as they are plainly

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
