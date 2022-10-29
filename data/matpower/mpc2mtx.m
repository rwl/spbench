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


[PQ, PV, REF, NONE, BUS_I, BUS_TYPE, PD, QD, GS, BS, BUS_AREA, VM, ...
    VA, BASE_KV, ZONE, VMAX, VMIN, LAM_P, LAM_Q, MU_VMAX, MU_VMIN] = idx_bus;
nb = size(mpc.bus, 1);
slack = find(mpc.bus(:, BUS_TYPE) == REF);
slack = slack(1);
noref   = (2:nb)';
noslack = find((1:nb)' ~= slack);
Bbus = Bbus(noslack, noref);

Bname = strcat(casename, '_Bbus.mtx');
mmwrite(Bname, Bbus);
%gzip(Bname, 'mtx');