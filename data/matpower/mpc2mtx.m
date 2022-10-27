function mpc2mtx(casename)

mpce = loadcase(casename);
mpc = ext2int(mpce);

%Ybus = makeYbus(mpc.baseMVA, mpc.bus, mpc.branch);
[J, Ybus] = makeJac(mpc.baseMVA, mpc.bus, mpc.branch, mpc.gen, 0);

Jname = strcat(casename, '_Jac.mtx');
mmwrite(Jname, J);
gzip(Jname, 'mtx');

Yname = strcat(casename, '_Ybus.mtx');
mmwrite(Yname, Ybus);
%gzip(Yname, 'mtx');


Bbus = makeBdc(mpc.baseMVA, mpc.bus, mpc.branch);

Bname = strcat(casename, '_Bbus.mtx');
mmwrite(Bname, Bbus);
%gzip(Bname, 'mtx');