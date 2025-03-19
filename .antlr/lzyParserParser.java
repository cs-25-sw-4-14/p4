// Generated from /home/tbdlarsen/uni/sem4/p4/lzyParser by ANTLR 4.13.1
import org.antlr.v4.runtime.atn.*;
import org.antlr.v4.runtime.dfa.DFA;
import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.misc.*;
import org.antlr.v4.runtime.tree.*;
import java.util.List;
import java.util.Iterator;
import java.util.ArrayList;

@SuppressWarnings({"all", "warnings", "unchecked", "unused", "cast", "CheckReturnValue"})
public class lzyParserParser extends Parser {
	static { RuntimeMetaData.checkVersion("4.13.1", RuntimeMetaData.VERSION); }

	protected static final DFA[] _decisionToDFA;
	protected static final PredictionContextCache _sharedContextCache =
		new PredictionContextCache();
	public static final int
		T__0=1, T__1=2, T__2=3, T__3=4, T__4=5, T__5=6, T__6=7, T__7=8, T__8=9, 
		T__9=10, T__10=11, T__11=12, T__12=13, T__13=14, T__14=15, T__15=16, T__16=17, 
		T__17=18, T__18=19, T__19=20, T__20=21, T__21=22, T__22=23, T__23=24, 
		T__24=25, T__25=26, T__26=27, T__27=28, T__28=29, T__29=30, T__30=31, 
		T__31=32, T__32=33, T__33=34, T__34=35, T__35=36, ID=37, STRING=38, FLOAT=39, 
		INT=40, WS=41;
	public static final int
		RULE_program = 0, RULE_expr = 1, RULE_eqExpr = 2, RULE_relBigExpr = 3, 
		RULE_relSmallExpr = 4, RULE_arithExpr = 5, RULE_minusExpr = 6, RULE_plusExpr = 7, 
		RULE_divExpr = 8, RULE_multExpr = 9, RULE_modExpr = 10, RULE_boolTerm = 11, 
		RULE_aritExpr = 12, RULE_aritTerm = 13, RULE_type = 14, RULE_mutability = 15, 
		RULE_importance = 16;
	private static String[] makeRuleNames() {
		return new String[] {
			"program", "expr", "eqExpr", "relBigExpr", "relSmallExpr", "arithExpr", 
			"minusExpr", "plusExpr", "divExpr", "multExpr", "modExpr", "boolTerm", 
			"aritExpr", "aritTerm", "type", "mutability", "importance"
		};
	}
	public static final String[] ruleNames = makeRuleNames();

	private static String[] makeLiteralNames() {
		return new String[] {
			null, "'||'", "'=='", "'!='", "'>'", "'<'", "'-'", "'+'", "'/'", "'*'", 
			"'%'", "'true'", "'false'", "'^'", "'<<'", "'>>'", "'&'", "'|'", "'~'", 
			"'('", "')'", "'u8'", "'u16'", "'u32'", "'u64'", "'i8'", "'i16'", "'i32'", 
			"'i64'", "'f8'", "'f16'", "'f32'", "'f64'", "'bool'", "'string'", "'mut'", 
			"'triv'"
		};
	}
	private static final String[] _LITERAL_NAMES = makeLiteralNames();
	private static String[] makeSymbolicNames() {
		return new String[] {
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, null, null, null, null, null, null, null, null, null, null, null, 
			null, "ID", "STRING", "FLOAT", "INT", "WS"
		};
	}
	private static final String[] _SYMBOLIC_NAMES = makeSymbolicNames();
	public static final Vocabulary VOCABULARY = new VocabularyImpl(_LITERAL_NAMES, _SYMBOLIC_NAMES);

	/**
	 * @deprecated Use {@link #VOCABULARY} instead.
	 */
	@Deprecated
	public static final String[] tokenNames;
	static {
		tokenNames = new String[_SYMBOLIC_NAMES.length];
		for (int i = 0; i < tokenNames.length; i++) {
			tokenNames[i] = VOCABULARY.getLiteralName(i);
			if (tokenNames[i] == null) {
				tokenNames[i] = VOCABULARY.getSymbolicName(i);
			}

			if (tokenNames[i] == null) {
				tokenNames[i] = "<INVALID>";
			}
		}
	}

	@Override
	@Deprecated
	public String[] getTokenNames() {
		return tokenNames;
	}

	@Override

	public Vocabulary getVocabulary() {
		return VOCABULARY;
	}

	@Override
	public String getGrammarFileName() { return "lzyParser"; }

	@Override
	public String[] getRuleNames() { return ruleNames; }

	@Override
	public String getSerializedATN() { return _serializedATN; }

	@Override
	public ATN getATN() { return _ATN; }

	public lzyParserParser(TokenStream input) {
		super(input);
		_interp = new ParserATNSimulator(this,_ATN,_decisionToDFA,_sharedContextCache);
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ProgramContext extends ParserRuleContext {
		public ProgramContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_program; }
	}

	public final ProgramContext program() throws RecognitionException {
		ProgramContext _localctx = new ProgramContext(_ctx, getState());
		enterRule(_localctx, 0, RULE_program);
		try {
			enterOuterAlt(_localctx, 1);
			{
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ExprContext extends ParserRuleContext {
		public List<EqExprContext> eqExpr() {
			return getRuleContexts(EqExprContext.class);
		}
		public EqExprContext eqExpr(int i) {
			return getRuleContext(EqExprContext.class,i);
		}
		public ExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_expr; }
	}

	public final ExprContext expr() throws RecognitionException {
		ExprContext _localctx = new ExprContext(_ctx, getState());
		enterRule(_localctx, 2, RULE_expr);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(36);
			eqExpr();
			setState(41);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==T__0) {
				{
				{
				setState(37);
				match(T__0);
				setState(38);
				eqExpr();
				}
				}
				setState(43);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class EqExprContext extends ParserRuleContext {
		public List<RelBigExprContext> relBigExpr() {
			return getRuleContexts(RelBigExprContext.class);
		}
		public RelBigExprContext relBigExpr(int i) {
			return getRuleContext(RelBigExprContext.class,i);
		}
		public EqExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_eqExpr; }
	}

	public final EqExprContext eqExpr() throws RecognitionException {
		EqExprContext _localctx = new EqExprContext(_ctx, getState());
		enterRule(_localctx, 4, RULE_eqExpr);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(44);
			relBigExpr();
			setState(49);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==T__1 || _la==T__2) {
				{
				{
				setState(45);
				_la = _input.LA(1);
				if ( !(_la==T__1 || _la==T__2) ) {
				_errHandler.recoverInline(this);
				}
				else {
					if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
					_errHandler.reportMatch(this);
					consume();
				}
				setState(46);
				relBigExpr();
				}
				}
				setState(51);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class RelBigExprContext extends ParserRuleContext {
		public List<RelSmallExprContext> relSmallExpr() {
			return getRuleContexts(RelSmallExprContext.class);
		}
		public RelSmallExprContext relSmallExpr(int i) {
			return getRuleContext(RelSmallExprContext.class,i);
		}
		public RelBigExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_relBigExpr; }
	}

	public final RelBigExprContext relBigExpr() throws RecognitionException {
		RelBigExprContext _localctx = new RelBigExprContext(_ctx, getState());
		enterRule(_localctx, 6, RULE_relBigExpr);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(52);
			relSmallExpr();
			setState(57);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==T__3) {
				{
				{
				setState(53);
				match(T__3);
				setState(54);
				relSmallExpr();
				}
				}
				setState(59);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class RelSmallExprContext extends ParserRuleContext {
		public List<PlusExprContext> plusExpr() {
			return getRuleContexts(PlusExprContext.class);
		}
		public PlusExprContext plusExpr(int i) {
			return getRuleContext(PlusExprContext.class,i);
		}
		public RelSmallExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_relSmallExpr; }
	}

	public final RelSmallExprContext relSmallExpr() throws RecognitionException {
		RelSmallExprContext _localctx = new RelSmallExprContext(_ctx, getState());
		enterRule(_localctx, 8, RULE_relSmallExpr);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(60);
			plusExpr();
			setState(65);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==T__4) {
				{
				{
				setState(61);
				match(T__4);
				setState(62);
				plusExpr();
				}
				}
				setState(67);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ArithExprContext extends ParserRuleContext {
		public MinusExprContext minusExpr() {
			return getRuleContext(MinusExprContext.class,0);
		}
		public ArithExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_arithExpr; }
	}

	public final ArithExprContext arithExpr() throws RecognitionException {
		ArithExprContext _localctx = new ArithExprContext(_ctx, getState());
		enterRule(_localctx, 10, RULE_arithExpr);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(68);
			minusExpr();
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MinusExprContext extends ParserRuleContext {
		public List<PlusExprContext> plusExpr() {
			return getRuleContexts(PlusExprContext.class);
		}
		public PlusExprContext plusExpr(int i) {
			return getRuleContext(PlusExprContext.class,i);
		}
		public MinusExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_minusExpr; }
	}

	public final MinusExprContext minusExpr() throws RecognitionException {
		MinusExprContext _localctx = new MinusExprContext(_ctx, getState());
		enterRule(_localctx, 12, RULE_minusExpr);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(70);
			plusExpr();
			setState(75);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==T__5) {
				{
				{
				setState(71);
				match(T__5);
				setState(72);
				plusExpr();
				}
				}
				setState(77);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class PlusExprContext extends ParserRuleContext {
		public List<DivExprContext> divExpr() {
			return getRuleContexts(DivExprContext.class);
		}
		public DivExprContext divExpr(int i) {
			return getRuleContext(DivExprContext.class,i);
		}
		public PlusExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_plusExpr; }
	}

	public final PlusExprContext plusExpr() throws RecognitionException {
		PlusExprContext _localctx = new PlusExprContext(_ctx, getState());
		enterRule(_localctx, 14, RULE_plusExpr);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(78);
			divExpr();
			setState(83);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==T__6) {
				{
				{
				setState(79);
				match(T__6);
				setState(80);
				divExpr();
				}
				}
				setState(85);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class DivExprContext extends ParserRuleContext {
		public List<MultExprContext> multExpr() {
			return getRuleContexts(MultExprContext.class);
		}
		public MultExprContext multExpr(int i) {
			return getRuleContext(MultExprContext.class,i);
		}
		public DivExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_divExpr; }
	}

	public final DivExprContext divExpr() throws RecognitionException {
		DivExprContext _localctx = new DivExprContext(_ctx, getState());
		enterRule(_localctx, 16, RULE_divExpr);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(86);
			multExpr();
			setState(91);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==T__7) {
				{
				{
				setState(87);
				match(T__7);
				setState(88);
				multExpr();
				}
				}
				setState(93);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MultExprContext extends ParserRuleContext {
		public List<ModExprContext> modExpr() {
			return getRuleContexts(ModExprContext.class);
		}
		public ModExprContext modExpr(int i) {
			return getRuleContext(ModExprContext.class,i);
		}
		public MultExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_multExpr; }
	}

	public final MultExprContext multExpr() throws RecognitionException {
		MultExprContext _localctx = new MultExprContext(_ctx, getState());
		enterRule(_localctx, 18, RULE_multExpr);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(94);
			modExpr();
			setState(99);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==T__8) {
				{
				{
				setState(95);
				match(T__8);
				setState(96);
				modExpr();
				}
				}
				setState(101);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ModExprContext extends ParserRuleContext {
		public List<TypeContext> type() {
			return getRuleContexts(TypeContext.class);
		}
		public TypeContext type(int i) {
			return getRuleContext(TypeContext.class,i);
		}
		public ModExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_modExpr; }
	}

	public final ModExprContext modExpr() throws RecognitionException {
		ModExprContext _localctx = new ModExprContext(_ctx, getState());
		enterRule(_localctx, 20, RULE_modExpr);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(102);
			type();
			setState(107);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==T__9) {
				{
				{
				setState(103);
				match(T__9);
				setState(104);
				type();
				}
				}
				setState(109);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class BoolTermContext extends ParserRuleContext {
		public TerminalNode ID() { return getToken(lzyParserParser.ID, 0); }
		public BoolTermContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_boolTerm; }
	}

	public final BoolTermContext boolTerm() throws RecognitionException {
		BoolTermContext _localctx = new BoolTermContext(_ctx, getState());
		enterRule(_localctx, 22, RULE_boolTerm);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(110);
			_la = _input.LA(1);
			if ( !((((_la) & ~0x3f) == 0 && ((1L << _la) & 137438959616L) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AritExprContext extends ParserRuleContext {
		public List<AritTermContext> aritTerm() {
			return getRuleContexts(AritTermContext.class);
		}
		public AritTermContext aritTerm(int i) {
			return getRuleContext(AritTermContext.class,i);
		}
		public AritExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_aritExpr; }
	}

	public final AritExprContext aritExpr() throws RecognitionException {
		AritExprContext _localctx = new AritExprContext(_ctx, getState());
		enterRule(_localctx, 24, RULE_aritExpr);
		try {
			setState(157);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,9,_ctx) ) {
			case 1:
				enterOuterAlt(_localctx, 1);
				{
				setState(112);
				aritTerm();
				}
				break;
			case 2:
				enterOuterAlt(_localctx, 2);
				{
				setState(113);
				aritTerm();
				setState(114);
				match(T__6);
				setState(115);
				aritTerm();
				}
				break;
			case 3:
				enterOuterAlt(_localctx, 3);
				{
				setState(117);
				aritTerm();
				setState(118);
				match(T__5);
				setState(119);
				aritTerm();
				}
				break;
			case 4:
				enterOuterAlt(_localctx, 4);
				{
				setState(121);
				aritTerm();
				setState(122);
				match(T__8);
				setState(123);
				aritTerm();
				}
				break;
			case 5:
				enterOuterAlt(_localctx, 5);
				{
				setState(125);
				aritTerm();
				setState(126);
				match(T__7);
				setState(127);
				aritTerm();
				}
				break;
			case 6:
				enterOuterAlt(_localctx, 6);
				{
				setState(129);
				aritTerm();
				setState(130);
				match(T__9);
				setState(131);
				aritTerm();
				}
				break;
			case 7:
				enterOuterAlt(_localctx, 7);
				{
				setState(133);
				aritTerm();
				setState(134);
				match(T__12);
				setState(135);
				aritTerm();
				}
				break;
			case 8:
				enterOuterAlt(_localctx, 8);
				{
				setState(137);
				aritTerm();
				setState(138);
				match(T__13);
				setState(139);
				aritTerm();
				}
				break;
			case 9:
				enterOuterAlt(_localctx, 9);
				{
				setState(141);
				aritTerm();
				setState(142);
				match(T__14);
				setState(143);
				aritTerm();
				}
				break;
			case 10:
				enterOuterAlt(_localctx, 10);
				{
				setState(145);
				aritTerm();
				setState(146);
				match(T__15);
				setState(147);
				aritTerm();
				}
				break;
			case 11:
				enterOuterAlt(_localctx, 11);
				{
				setState(149);
				aritTerm();
				setState(150);
				match(T__16);
				setState(151);
				aritTerm();
				}
				break;
			case 12:
				enterOuterAlt(_localctx, 12);
				{
				setState(153);
				aritTerm();
				setState(154);
				match(T__17);
				setState(155);
				aritTerm();
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class AritTermContext extends ParserRuleContext {
		public AritExprContext aritExpr() {
			return getRuleContext(AritExprContext.class,0);
		}
		public TypeContext type() {
			return getRuleContext(TypeContext.class,0);
		}
		public TerminalNode ID() { return getToken(lzyParserParser.ID, 0); }
		public AritTermContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_aritTerm; }
	}

	public final AritTermContext aritTerm() throws RecognitionException {
		AritTermContext _localctx = new AritTermContext(_ctx, getState());
		enterRule(_localctx, 26, RULE_aritTerm);
		try {
			setState(165);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case T__18:
				enterOuterAlt(_localctx, 1);
				{
				setState(159);
				match(T__18);
				setState(160);
				aritExpr();
				setState(161);
				match(T__19);
				}
				break;
			case T__20:
			case T__21:
			case T__22:
			case T__23:
			case T__24:
			case T__25:
			case T__26:
			case T__27:
			case T__28:
			case T__29:
			case T__30:
			case T__31:
			case T__32:
			case T__33:
				enterOuterAlt(_localctx, 2);
				{
				setState(163);
				type();
				}
				break;
			case ID:
				enterOuterAlt(_localctx, 3);
				{
				setState(164);
				match(ID);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class TypeContext extends ParserRuleContext {
		public TypeContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_type; }
	}

	public final TypeContext type() throws RecognitionException {
		TypeContext _localctx = new TypeContext(_ctx, getState());
		enterRule(_localctx, 28, RULE_type);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(167);
			_la = _input.LA(1);
			if ( !((((_la) & ~0x3f) == 0 && ((1L << _la) & 34357641216L) != 0)) ) {
			_errHandler.recoverInline(this);
			}
			else {
				if ( _input.LA(1)==Token.EOF ) matchedEOF = true;
				_errHandler.reportMatch(this);
				consume();
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class MutabilityContext extends ParserRuleContext {
		public MutabilityContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mutability; }
	}

	public final MutabilityContext mutability() throws RecognitionException {
		MutabilityContext _localctx = new MutabilityContext(_ctx, getState());
		enterRule(_localctx, 30, RULE_mutability);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(169);
			match(T__34);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	@SuppressWarnings("CheckReturnValue")
	public static class ImportanceContext extends ParserRuleContext {
		public ImportanceContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_importance; }
	}

	public final ImportanceContext importance() throws RecognitionException {
		ImportanceContext _localctx = new ImportanceContext(_ctx, getState());
		enterRule(_localctx, 32, RULE_importance);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(171);
			match(T__35);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static final String _serializedATN =
		"\u0004\u0001)\u00ae\u0002\u0000\u0007\u0000\u0002\u0001\u0007\u0001\u0002"+
		"\u0002\u0007\u0002\u0002\u0003\u0007\u0003\u0002\u0004\u0007\u0004\u0002"+
		"\u0005\u0007\u0005\u0002\u0006\u0007\u0006\u0002\u0007\u0007\u0007\u0002"+
		"\b\u0007\b\u0002\t\u0007\t\u0002\n\u0007\n\u0002\u000b\u0007\u000b\u0002"+
		"\f\u0007\f\u0002\r\u0007\r\u0002\u000e\u0007\u000e\u0002\u000f\u0007\u000f"+
		"\u0002\u0010\u0007\u0010\u0001\u0000\u0001\u0000\u0001\u0001\u0001\u0001"+
		"\u0001\u0001\u0005\u0001(\b\u0001\n\u0001\f\u0001+\t\u0001\u0001\u0002"+
		"\u0001\u0002\u0001\u0002\u0005\u00020\b\u0002\n\u0002\f\u00023\t\u0002"+
		"\u0001\u0003\u0001\u0003\u0001\u0003\u0005\u00038\b\u0003\n\u0003\f\u0003"+
		";\t\u0003\u0001\u0004\u0001\u0004\u0001\u0004\u0005\u0004@\b\u0004\n\u0004"+
		"\f\u0004C\t\u0004\u0001\u0005\u0001\u0005\u0001\u0006\u0001\u0006\u0001"+
		"\u0006\u0005\u0006J\b\u0006\n\u0006\f\u0006M\t\u0006\u0001\u0007\u0001"+
		"\u0007\u0001\u0007\u0005\u0007R\b\u0007\n\u0007\f\u0007U\t\u0007\u0001"+
		"\b\u0001\b\u0001\b\u0005\bZ\b\b\n\b\f\b]\t\b\u0001\t\u0001\t\u0001\t\u0005"+
		"\tb\b\t\n\t\f\te\t\t\u0001\n\u0001\n\u0001\n\u0005\nj\b\n\n\n\f\nm\t\n"+
		"\u0001\u000b\u0001\u000b\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001"+
		"\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001"+
		"\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001"+
		"\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001"+
		"\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001\f\u0001"+
		"\f\u0001\f\u0001\f\u0001\f\u0003\f\u009e\b\f\u0001\r\u0001\r\u0001\r\u0001"+
		"\r\u0001\r\u0001\r\u0003\r\u00a6\b\r\u0001\u000e\u0001\u000e\u0001\u000f"+
		"\u0001\u000f\u0001\u0010\u0001\u0010\u0001\u0010\u0000\u0000\u0011\u0000"+
		"\u0002\u0004\u0006\b\n\f\u000e\u0010\u0012\u0014\u0016\u0018\u001a\u001c"+
		"\u001e \u0000\u0003\u0001\u0000\u0002\u0003\u0002\u0000\u000b\f%%\u0001"+
		"\u0000\u0015\"\u00b2\u0000\"\u0001\u0000\u0000\u0000\u0002$\u0001\u0000"+
		"\u0000\u0000\u0004,\u0001\u0000\u0000\u0000\u00064\u0001\u0000\u0000\u0000"+
		"\b<\u0001\u0000\u0000\u0000\nD\u0001\u0000\u0000\u0000\fF\u0001\u0000"+
		"\u0000\u0000\u000eN\u0001\u0000\u0000\u0000\u0010V\u0001\u0000\u0000\u0000"+
		"\u0012^\u0001\u0000\u0000\u0000\u0014f\u0001\u0000\u0000\u0000\u0016n"+
		"\u0001\u0000\u0000\u0000\u0018\u009d\u0001\u0000\u0000\u0000\u001a\u00a5"+
		"\u0001\u0000\u0000\u0000\u001c\u00a7\u0001\u0000\u0000\u0000\u001e\u00a9"+
		"\u0001\u0000\u0000\u0000 \u00ab\u0001\u0000\u0000\u0000\"#\u0001\u0000"+
		"\u0000\u0000#\u0001\u0001\u0000\u0000\u0000$)\u0003\u0004\u0002\u0000"+
		"%&\u0005\u0001\u0000\u0000&(\u0003\u0004\u0002\u0000\'%\u0001\u0000\u0000"+
		"\u0000(+\u0001\u0000\u0000\u0000)\'\u0001\u0000\u0000\u0000)*\u0001\u0000"+
		"\u0000\u0000*\u0003\u0001\u0000\u0000\u0000+)\u0001\u0000\u0000\u0000"+
		",1\u0003\u0006\u0003\u0000-.\u0007\u0000\u0000\u0000.0\u0003\u0006\u0003"+
		"\u0000/-\u0001\u0000\u0000\u000003\u0001\u0000\u0000\u00001/\u0001\u0000"+
		"\u0000\u000012\u0001\u0000\u0000\u00002\u0005\u0001\u0000\u0000\u0000"+
		"31\u0001\u0000\u0000\u000049\u0003\b\u0004\u000056\u0005\u0004\u0000\u0000"+
		"68\u0003\b\u0004\u000075\u0001\u0000\u0000\u00008;\u0001\u0000\u0000\u0000"+
		"97\u0001\u0000\u0000\u00009:\u0001\u0000\u0000\u0000:\u0007\u0001\u0000"+
		"\u0000\u0000;9\u0001\u0000\u0000\u0000<A\u0003\u000e\u0007\u0000=>\u0005"+
		"\u0005\u0000\u0000>@\u0003\u000e\u0007\u0000?=\u0001\u0000\u0000\u0000"+
		"@C\u0001\u0000\u0000\u0000A?\u0001\u0000\u0000\u0000AB\u0001\u0000\u0000"+
		"\u0000B\t\u0001\u0000\u0000\u0000CA\u0001\u0000\u0000\u0000DE\u0003\f"+
		"\u0006\u0000E\u000b\u0001\u0000\u0000\u0000FK\u0003\u000e\u0007\u0000"+
		"GH\u0005\u0006\u0000\u0000HJ\u0003\u000e\u0007\u0000IG\u0001\u0000\u0000"+
		"\u0000JM\u0001\u0000\u0000\u0000KI\u0001\u0000\u0000\u0000KL\u0001\u0000"+
		"\u0000\u0000L\r\u0001\u0000\u0000\u0000MK\u0001\u0000\u0000\u0000NS\u0003"+
		"\u0010\b\u0000OP\u0005\u0007\u0000\u0000PR\u0003\u0010\b\u0000QO\u0001"+
		"\u0000\u0000\u0000RU\u0001\u0000\u0000\u0000SQ\u0001\u0000\u0000\u0000"+
		"ST\u0001\u0000\u0000\u0000T\u000f\u0001\u0000\u0000\u0000US\u0001\u0000"+
		"\u0000\u0000V[\u0003\u0012\t\u0000WX\u0005\b\u0000\u0000XZ\u0003\u0012"+
		"\t\u0000YW\u0001\u0000\u0000\u0000Z]\u0001\u0000\u0000\u0000[Y\u0001\u0000"+
		"\u0000\u0000[\\\u0001\u0000\u0000\u0000\\\u0011\u0001\u0000\u0000\u0000"+
		"][\u0001\u0000\u0000\u0000^c\u0003\u0014\n\u0000_`\u0005\t\u0000\u0000"+
		"`b\u0003\u0014\n\u0000a_\u0001\u0000\u0000\u0000be\u0001\u0000\u0000\u0000"+
		"ca\u0001\u0000\u0000\u0000cd\u0001\u0000\u0000\u0000d\u0013\u0001\u0000"+
		"\u0000\u0000ec\u0001\u0000\u0000\u0000fk\u0003\u001c\u000e\u0000gh\u0005"+
		"\n\u0000\u0000hj\u0003\u001c\u000e\u0000ig\u0001\u0000\u0000\u0000jm\u0001"+
		"\u0000\u0000\u0000ki\u0001\u0000\u0000\u0000kl\u0001\u0000\u0000\u0000"+
		"l\u0015\u0001\u0000\u0000\u0000mk\u0001\u0000\u0000\u0000no\u0007\u0001"+
		"\u0000\u0000o\u0017\u0001\u0000\u0000\u0000p\u009e\u0003\u001a\r\u0000"+
		"qr\u0003\u001a\r\u0000rs\u0005\u0007\u0000\u0000st\u0003\u001a\r\u0000"+
		"t\u009e\u0001\u0000\u0000\u0000uv\u0003\u001a\r\u0000vw\u0005\u0006\u0000"+
		"\u0000wx\u0003\u001a\r\u0000x\u009e\u0001\u0000\u0000\u0000yz\u0003\u001a"+
		"\r\u0000z{\u0005\t\u0000\u0000{|\u0003\u001a\r\u0000|\u009e\u0001\u0000"+
		"\u0000\u0000}~\u0003\u001a\r\u0000~\u007f\u0005\b\u0000\u0000\u007f\u0080"+
		"\u0003\u001a\r\u0000\u0080\u009e\u0001\u0000\u0000\u0000\u0081\u0082\u0003"+
		"\u001a\r\u0000\u0082\u0083\u0005\n\u0000\u0000\u0083\u0084\u0003\u001a"+
		"\r\u0000\u0084\u009e\u0001\u0000\u0000\u0000\u0085\u0086\u0003\u001a\r"+
		"\u0000\u0086\u0087\u0005\r\u0000\u0000\u0087\u0088\u0003\u001a\r\u0000"+
		"\u0088\u009e\u0001\u0000\u0000\u0000\u0089\u008a\u0003\u001a\r\u0000\u008a"+
		"\u008b\u0005\u000e\u0000\u0000\u008b\u008c\u0003\u001a\r\u0000\u008c\u009e"+
		"\u0001\u0000\u0000\u0000\u008d\u008e\u0003\u001a\r\u0000\u008e\u008f\u0005"+
		"\u000f\u0000\u0000\u008f\u0090\u0003\u001a\r\u0000\u0090\u009e\u0001\u0000"+
		"\u0000\u0000\u0091\u0092\u0003\u001a\r\u0000\u0092\u0093\u0005\u0010\u0000"+
		"\u0000\u0093\u0094\u0003\u001a\r\u0000\u0094\u009e\u0001\u0000\u0000\u0000"+
		"\u0095\u0096\u0003\u001a\r\u0000\u0096\u0097\u0005\u0011\u0000\u0000\u0097"+
		"\u0098\u0003\u001a\r\u0000\u0098\u009e\u0001\u0000\u0000\u0000\u0099\u009a"+
		"\u0003\u001a\r\u0000\u009a\u009b\u0005\u0012\u0000\u0000\u009b\u009c\u0003"+
		"\u001a\r\u0000\u009c\u009e\u0001\u0000\u0000\u0000\u009dp\u0001\u0000"+
		"\u0000\u0000\u009dq\u0001\u0000\u0000\u0000\u009du\u0001\u0000\u0000\u0000"+
		"\u009dy\u0001\u0000\u0000\u0000\u009d}\u0001\u0000\u0000\u0000\u009d\u0081"+
		"\u0001\u0000\u0000\u0000\u009d\u0085\u0001\u0000\u0000\u0000\u009d\u0089"+
		"\u0001\u0000\u0000\u0000\u009d\u008d\u0001\u0000\u0000\u0000\u009d\u0091"+
		"\u0001\u0000\u0000\u0000\u009d\u0095\u0001\u0000\u0000\u0000\u009d\u0099"+
		"\u0001\u0000\u0000\u0000\u009e\u0019\u0001\u0000\u0000\u0000\u009f\u00a0"+
		"\u0005\u0013\u0000\u0000\u00a0\u00a1\u0003\u0018\f\u0000\u00a1\u00a2\u0005"+
		"\u0014\u0000\u0000\u00a2\u00a6\u0001\u0000\u0000\u0000\u00a3\u00a6\u0003"+
		"\u001c\u000e\u0000\u00a4\u00a6\u0005%\u0000\u0000\u00a5\u009f\u0001\u0000"+
		"\u0000\u0000\u00a5\u00a3\u0001\u0000\u0000\u0000\u00a5\u00a4\u0001\u0000"+
		"\u0000\u0000\u00a6\u001b\u0001\u0000\u0000\u0000\u00a7\u00a8\u0007\u0002"+
		"\u0000\u0000\u00a8\u001d\u0001\u0000\u0000\u0000\u00a9\u00aa\u0005#\u0000"+
		"\u0000\u00aa\u001f\u0001\u0000\u0000\u0000\u00ab\u00ac\u0005$\u0000\u0000"+
		"\u00ac!\u0001\u0000\u0000\u0000\u000b)19AKS[ck\u009d\u00a5";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}