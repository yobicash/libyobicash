use crypto::elliptic::scalar::YScalar;
use crypto::elliptic::point::*;

#[derive(Clone, Debug, Default)]
pub struct YSchnorrProtocolParams {
    g: Option<YPoint>,
    x: Option<YScalar>,
    u: Option<YScalar>,
}

impl YSchnorrProtocolParams {
    pub fn random() -> YSchnorrProtocolParams {
        YSchnorrProtocolParams {
            g: Some(YPoint::default()),
            x: Some(YScalar::random()),
            u: Some(YScalar::random()),
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct YSchnorrProtocol {
    pub g: YPoint, // generator g
    pub x: YScalar, // instance x
    pub w: YPoint, // witness w = g^x
    pub u: YScalar, // random u
    pub t: YPoint, // commit t = g^u
    pub c: YScalar, // challenge c = H(g, t)
    pub r: YScalar, // reply r = u + cx
}

impl YSchnorrProtocol {
    pub fn from_params(p: &YSchnorrProtocolParams) -> YSchnorrProtocol {
        let mut prot = YSchnorrProtocol::default();

        if let Some(_g) = p.g {
            prot.g = _g;
        } else {
            prot.g = YPoint::default();
        }

        if let Some(_x) = p.x {
            prot.x = _x;
        } else {
            prot.x = YScalar::random();
        }

        prot.w = &prot.g * &prot.x;

        if let Some(_u) = p.u {
            prot.u = _u;
        } else {
            prot.u = YScalar::random();
        }

        prot.t = &prot.g * &prot.u;

        let mut g_bin: Vec<u8> = Vec::new();
        g_bin.extend_from_slice(&prot.g.to_bytes()[..]);

        let mut w_bin: Vec<u8> = Vec::new();
        w_bin.extend_from_slice(&prot.w.to_bytes()[..]);

        let mut t_bin: Vec<u8> = Vec::new();
        t_bin.extend_from_slice(&prot.t.to_bytes()[..]);

        let mut c_bin: Vec<u8> = Vec::new();
        c_bin.extend_from_slice(g_bin.as_slice());
        c_bin.extend_from_slice(w_bin.as_slice());
        c_bin.extend_from_slice(t_bin.as_slice());

        prot.c = YScalar::hash_from_bytes(c_bin.as_slice());

        prot.r = &prot.u + &(&prot.x * &prot.c);

        prot
    }

    pub fn random() -> YSchnorrProtocol {
        YSchnorrProtocol::from_params(&YSchnorrProtocolParams::random())
    }

    pub fn to_public(&self) -> YSchnorrProtocolPublic {
        YSchnorrProtocolPublic {
            g: self.g,
            w: self.w,
            t: self.t,
            c: self.c,
            r: self.r,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct YSchnorrProtocolPublic {
    pub g: YPoint,
    pub w: YPoint,
    pub t: YPoint,
    pub c: YScalar,
    pub r: YScalar,
}

impl YSchnorrProtocolPublic {
    pub fn verify(&self) -> bool {
        let gr = &self.g * &self.r;
        let twc = &self.t + &(&self.w * &self.c);
        gr == twc
    }
}
