core-musl-std-objs += string/memchr.o string/memcmp.o string/memcpy.o string/memmove.o \
	string/memrchr.o string/memset.o string/strcat.o string/strchr.o string/strchrnul.o \
	string/strcmp.o string/strcpy.o string/strlen.o string/strncat.o string/strncmp.o \
	string/strncpy.o string/strrchr.o string/strstr.o string/stpcpy.o string/stpncpy.o \
	string/strcspn.o string/strpbrk.o string/strspn.o string/strtok.o

core-musl-std-objs += math/acos.o math/acosf.o math/acosh.o math/acoshf.o math/acoshl.o \
	math/acosl.o math/asin.o math/asinf.o math/asinh.o math/asinhf.o math/asinhl.o \
	math/asinl.o math/atan2.o math/atan2f.o math/atan2l.o math/atan.o math/atanf.o \
	math/atanh.o math/atanhf.o math/atanhl.o math/atanl.o math/cbrt.o math/cbrtf.o \
	math/cbrtl.o math/ceil.o math/ceilf.o math/ceill.o math/copysign.o math/copysignf.o \
	math/copysignl.o math/cos.o math/cosf.o math/cosh.o math/coshf.o math/coshl.o \
	math/cosl.o math/erf.o math/erff.o math/erfl.o math/exp10.o math/exp10f.o \
	math/exp10l.o math/exp2.o math/exp2f.o math/exp2f_data.o math/exp2l.o math/exp.o \
	math/exp_data.o math/expf.o math/expl.o math/expm1.o math/expm1f.o math/expm1l.o \
	math/fabs.o math/fabsf.o math/fabsl.o math/fdim.o math/fdimf.o math/fdiml.o \
	math/finite.o math/finitef.o math/floor.o math/floorf.o math/floorl.o math/fma.o \
	math/fmax.o math/fmaxf.o math/fmaxl.o math/fmin.o \
	math/fminf.o math/fminl.o math/fmod.o math/fmodf.o math/fmodl.o math/frexp.o \
	math/frexpf.o math/frexpl.o math/hypot.o math/hypotf.o math/hypotl.o math/ilogb.o \
	math/ilogbf.o math/ilogbl.o math/j0.o math/j0f.o math/j1.o math/j1f.o math/jn.o \
	math/jnf.o math/ldexp.o math/ldexpf.o math/ldexpl.o math/lgamma.o math/lgammaf.o \
	math/lgammaf_r.o math/lgammal.o math/lgamma_r.o math/llrint.o math/llrintf.o \
	math/llrintl.o math/llround.o math/llroundf.o math/llroundl.o math/log10.o \
	math/log10f.o math/log10l.o math/log1p.o math/log1pf.o math/log1pl.o math/log2.o \
	math/log2_data.o math/log2f.o math/log2f_data.o math/log2l.o math/logb.o math/logbf.o \
	math/logbl.o math/log.o math/log_data.o math/logf.o math/logf_data.o math/logl.o \
	math/lrint.o math/lrintf.o math/lrintl.o math/lround.o math/lroundf.o math/lroundl.o \
	math/modf.o math/modff.o math/modfl.o math/nan.o math/nanf.o math/nanl.o math/nearbyint.o \
	math/nearbyintf.o math/nearbyintl.o math/nextafter.o math/nextafterf.o math/nextafterl.o \
	math/nexttoward.o math/nexttowardf.o math/nexttowardl.o math/pow.o math/pow_data.o math/powf.o \
	math/powf_data.o math/powl.o math/remainder.o math/remainderf.o math/remainderl.o \
	math/remquo.o math/remquof.o math/remquol.o math/rint.o math/rintf.o math/rintl.o \
	math/round.o math/roundf.o math/roundl.o math/scalb.o math/scalbf.o math/scalbln.o \
	math/scalblnf.o math/scalblnl.o math/scalbn.o math/scalbnf.o math/scalbnl.o math/signgam.o \
	math/significand.o math/significandf.o math/sin.o math/sincos.o math/sincosf.o math/sincosl.o \
	math/sinf.o math/sinh.o math/sinhf.o math/sinhl.o math/sinl.o math/sqrt.o math/sqrt_data.o \
	math/sqrtf.o math/sqrtl.o math/tan.o math/tanf.o math/tanh.o math/tanhf.o math/tanhl.o \
	math/tanl.o math/tgamma.o math/tgammaf.o math/tgammal.o math/trunc.o math/truncf.o math/truncl.o
# math/fmaf.o math/fmal.o

core-musl-std-objs += math/__cos.o math/__cosdf.o math/__cosl.o math/__expo2.o math/__expo2f.o \
	math/__fpclassify.o math/__fpclassifyf.o math/__fpclassifyl.o math/__invtrigl.o \
	math/__math_divzero.o math/__math_divzerof.o math/__math_invalid.o math/__math_invalidf.o \
	math/__math_invalidl.o math/__math_oflow.o math/__math_oflowf.o math/__math_uflow.o \
	math/__math_uflowf.o math/__math_xflow.o math/__math_xflowf.o math/__polevll.o \
	math/__rem_pio2.o math/__rem_pio2f.o math/__rem_pio2_large.o math/__rem_pio2l.o \
	math/__signbit.o math/__signbitf.o math/__signbitl.o math/__sin.o math/__sindf.o \
	math/__sinl.o math/__tan.o math/__tandf.o math/__tanl.o

$(call load-objs, core, $(foreach __muslobj,$(core-musl-std-objs),src/$(__muslobj)))

define rule_core_cxx_std_musl
$(eval $$(out)/core/cxx/std/musl/%: cflags += -include $$(src)/core/cxx/std/musl/base.h -isystem $$(musl-src)src/include/ -isystem $$(musl-src)src/internal/ -isystem $$(musl-src)include/)
endef

make-finalize += rule_core_cxx_std_musl
